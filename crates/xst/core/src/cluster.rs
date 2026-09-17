use core::{any::TypeId, marker::PhantomData};
use rustc_hash::FxHashMap;
use std::collections::{BTreeMap, BTreeSet};

use crate::shard::{ReferenceData, ShardData, ShardDataKind, StaticShard};
use mapping::Shape;
use optimize::{First, ShiftRange};
use set::CharSet;

mod forest;
mod mapping;
mod optimize;
mod set;

pub mod internal {
    pub use super::ExtractError;
    pub use super::MappingNode;
}

pub use forest::{ParseError, Parsed};
pub use mapping::{ExtractError, MappingNode};

pub struct Cluster<S>
where
    S: StaticShard,
{
    _shard: PhantomData<fn() -> S>,
    table: Table,
}

impl<S> Cluster<S>
where
    S: StaticShard,
{
    pub fn build() -> Self {
        let mut grammar = Grammar::default();
        let start = grammar.nonterminal();
        grammar.rule(start, vec![]);
        let root = grammar.reference(&ReferenceData::new::<S>());
        grammar.rules[0].rhs.push(Symbol::Nonterminal(root));
        Self {
            _shard: PhantomData,
            table: Table::build(grammar),
        }
    }

    /// Recognizes the complete input without allocating a parse forest.
    pub fn recognizes(&self, input: &str) -> bool {
        self.table.parse(input)
    }

    /// Parses the complete input, retaining derivations until the preferred result is mapped.
    pub fn parse<'c, 'i>(&'c self, input: &'i str) -> Result<Parsed<'c, 'i, S>, ParseError> {
        Parsed::new(self, input)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Symbol {
    Terminal(usize),
    Nonterminal(usize),
}

// The LHS identifies the original grammar node (or repetition helper). Its
// Shape and this production's branch are the reduction recipe; optimizer
// passes must preserve both even when recognition symbols are shared.
#[derive(Debug)]
struct Rule {
    lhs: usize,
    rhs: Vec<Symbol>,
    branch: usize,
}

#[derive(Default)]
struct Grammar {
    #[cfg(test)]
    unshared_terminals: bool,
    rules: Vec<Rule>,
    by_lhs: Vec<Vec<usize>>,
    shapes: Vec<Shape>,
    terminals: Vec<CharSet>,
    terminal_ids: FxHashMap<CharSet, usize>,
    references: FxHashMap<TypeId, usize>,
    nodes: FxHashMap<*const ShardData, usize>,
}

impl Grammar {
    fn terminal(&mut self, set: CharSet) -> usize {
        #[cfg(test)]
        if self.unshared_terminals {
            let id = self.terminals.len();
            self.terminals.push(set);
            return id;
        }
        *self.terminal_ids.entry(set.clone()).or_insert_with(|| {
            let id = self.terminals.len();
            self.terminals.push(set);
            id
        })
    }

    fn nonterminal(&mut self) -> usize {
        let id = self.by_lhs.len();
        self.by_lhs.push(vec![]);
        self.shapes.push(Shape::Transparent);
        id
    }

    fn rule(&mut self, lhs: usize, rhs: Vec<Symbol>) {
        let branch = self.by_lhs[lhs].len();
        self.by_lhs[lhs].push(self.rules.len());
        self.rules.push(Rule { lhs, rhs, branch });
    }

    fn reference(&mut self, reference: &ReferenceData) -> usize {
        if let Some(&id) = self.references.get(&reference.id) {
            return id;
        }
        let id = self.nonterminal();
        self.references.insert(reference.id, id);
        self.shapes[id] = Shape::Reference(reference.id);
        let target = self.lower((reference.reference)());
        self.rule(id, vec![Symbol::Nonterminal(target)]);
        id
    }

    fn lower(&mut self, data: &'static ShardData) -> usize {
        if let Some(&id) = self.nodes.get(&(data as *const _)) {
            return id;
        }
        let id = self.nonterminal();
        self.nodes.insert(data as *const _, id);
        self.shapes[id] = match &data.kind {
            ShardDataKind::Literal(_) | ShardDataKind::Set(_) => Shape::Span,
            ShardDataKind::Reference(_) => Shape::Transparent,
            ShardDataKind::Sequence(_) => Shape::Sequence,
            ShardDataKind::Alternative(_) => Shape::Alternative,
            ShardDataKind::Option(_) => Shape::Option,
            ShardDataKind::Vec(_) => Shape::Repeat,
        };
        match &data.kind {
            ShardDataKind::Literal(literal) => {
                let rhs = literal
                    .text
                    .chars()
                    .map(|ch| {
                        let terminal = self.terminal(CharSet::new([(ch as u32, ch as u32)]));
                        Symbol::Terminal(terminal)
                    })
                    .collect();
                self.rule(id, rhs);
            }
            ShardDataKind::Set(set) => {
                let terminal = self.terminal(CharSet::from_data(set));
                self.rule(id, vec![Symbol::Terminal(terminal)]);
            }
            ShardDataKind::Reference(reference) => {
                let target = self.reference(reference);
                self.rule(id, vec![Symbol::Nonterminal(target)]);
            }
            ShardDataKind::Sequence(sequence) => {
                let rhs = sequence
                    .items
                    .iter()
                    .map(|item| Symbol::Nonterminal(self.lower(item)))
                    .collect();
                self.rule(id, rhs);
            }
            ShardDataKind::Alternative(alternative) => {
                for item in alternative.items {
                    let target = self.lower(item);
                    self.rule(id, vec![Symbol::Nonterminal(target)]);
                }
            }
            ShardDataKind::Option(option) => {
                let item = self.lower(option.item);
                self.optional(id, vec![Symbol::Nonterminal(item)], option.lazy);
            }
            ShardDataKind::Vec(repeat) => {
                assert!(
                    repeat.max.is_none_or(|max| max >= repeat.min),
                    "invalid repetition bounds"
                );
                let item = Symbol::Nonterminal(self.lower(repeat.item));
                let mut tail = id;
                for _ in 0..repeat.min {
                    let next = self.nonterminal();
                    self.shapes[next] = Shape::Repeat;
                    self.rule(tail, vec![item, Symbol::Nonterminal(next)]);
                    tail = next;
                }
                if let Some(max) = repeat.max {
                    for _ in repeat.min..max {
                        let next = self.nonterminal();
                        self.shapes[next] = Shape::Repeat;
                        self.optional(tail, vec![item, Symbol::Nonterminal(next)], repeat.lazy);
                        tail = next;
                    }
                    self.rule(tail, vec![]);
                } else {
                    self.optional(tail, vec![item, Symbol::Nonterminal(tail)], repeat.lazy);
                }
            }
        }
        id
    }

    fn optional(&mut self, lhs: usize, rhs: Vec<Symbol>, lazy: bool) {
        if lazy {
            self.rule(lhs, vec![]);
            self.rule(lhs, rhs);
        } else {
            self.rule(lhs, rhs);
            self.rule(lhs, vec![]);
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Item {
    rule: usize,
    dot: usize,
}

#[derive(Debug, Default)]
struct State {
    // Retained only for differential tests against predicate-based dispatch.
    #[cfg(test)]
    shifts: BTreeMap<usize, usize>,
    gotos: BTreeMap<usize, usize>,
    // LR(0) reductions apply on every lookahead, including EOF. Multiple
    // reductions and shifts coexist rather than resolving conflicts.
    reductions: Vec<usize>,
    accept: bool,
    first: First,
    dispatch: Vec<ShiftRange>,
    shift_symbols: BTreeMap<usize, usize>,
}

#[derive(Debug)]
struct Table {
    rules: Vec<Rule>,
    shapes: Vec<Shape>,
    states: Vec<State>,
}

// Graph-structured stack: each node represents one (input position, LR state).
// Edges point to predecessors. Labels and derivations are unnecessary until
// result extraction is implemented; recognition only needs the stack states.
struct StackNode {
    state: usize,
    predecessors: BTreeSet<usize>,
}

#[derive(Default)]
struct Stack {
    nodes: Vec<StackNode>,
    // Only nodes at the current input position can acquire new predecessors.
    current: BTreeMap<usize, usize>,
}

impl Stack {
    fn node(&mut self, state: usize) -> usize {
        *self.current.entry(state).or_insert_with(|| {
            let id = self.nodes.len();
            self.nodes.push(StackNode {
                state,
                predecessors: BTreeSet::new(),
            });
            id
        })
    }

    fn ancestors(&self, node: usize, depth: usize) -> BTreeSet<usize> {
        let mut frontier = BTreeSet::from([node]);
        // Deduplicate endpoints at each depth, rather than enumerate paths.
        // This also handles epsilon-induced cycles in the stack graph.
        for _ in 0..depth {
            frontier = frontier
                .iter()
                .flat_map(|&id| self.nodes[id].predecessors.iter().copied())
                .collect();
            if frontier.is_empty() {
                break;
            }
        }
        frontier
    }

    fn reduce(&mut self, table: &Table, lookahead: Option<char>) {
        loop {
            let mut changed = false;
            let heads: Vec<_> = self.current.values().copied().collect();
            for head in heads {
                for &rule in &table.states[self.nodes[head].state].reductions {
                    let rule = &table.rules[rule];
                    for ancestor in self.ancestors(head, rule.rhs.len()) {
                        let state = &table.states[self.nodes[ancestor].state];
                        if let Some(&target) = state.gotos.get(&rule.lhs) {
                            if !table.states[target].first.permits(lookahead) {
                                continue;
                            }
                            let node = self.node(target);
                            changed |= self.nodes[node].predecessors.insert(ancestor);
                        }
                    }
                }
            }
            // A new edge can expose reduction paths from existing heads, even
            // if those heads themselves did not change. Revisit all heads until
            // no edge is added. The finite set of nodes/edges guarantees exit.
            if !changed {
                break;
            }
        }
    }
}

impl Table {
    fn parse(&self, input: &str) -> bool {
        let mut stack = Stack::default();
        stack.node(0);
        let mut chars = input.chars().peekable();
        if !self.states[0].first.permits(chars.peek().copied()) {
            return false;
        }
        while let Some(ch) = chars.next() {
            stack.reduce(self, Some(ch));
            let heads = core::mem::take(&mut stack.current);
            for (state, head) in heads {
                for &target in ShiftRange::lookup(&self.states[state].dispatch, ch) {
                    if self.states[target].first.permits(chars.peek().copied()) {
                        let node = stack.node(target);
                        stack.nodes[node].predecessors.insert(head);
                    }
                }
            }
            if stack.current.is_empty() {
                return false;
            }
        }
        stack.reduce(self, None);
        stack.current.keys().any(|&state| self.states[state].accept)
    }

    fn closure(grammar: &Grammar, seeds: impl IntoIterator<Item = Item>) -> Vec<Item> {
        let mut items = BTreeSet::new();
        let mut pending = Vec::new();
        for item in seeds {
            if items.insert(item) {
                pending.push(item);
            }
        }
        while let Some(item) = pending.pop() {
            if let Some(Symbol::Nonterminal(lhs)) = grammar.rules[item.rule].rhs.get(item.dot) {
                for &rule in &grammar.by_lhs[*lhs] {
                    let next = Item { rule, dot: 0 };
                    if items.insert(next) {
                        pending.push(next);
                    }
                }
            }
        }
        items.into_iter().collect()
    }

    fn build(grammar: Grammar) -> Self {
        let first = First::build(&grammar);
        let initial = Self::closure(&grammar, [Item { rule: 0, dot: 0 }]);
        let mut known = FxHashMap::default();
        known.insert(initial.clone(), 0);
        let mut item_sets = vec![initial];
        let mut states = Vec::new();
        while states.len() < item_sets.len() {
            let mut state = State {
                first: First::state(&grammar, &first, &item_sets[states.len()]),
                ..State::default()
            };
            let mut shifts = BTreeMap::new();
            let mut transitions: BTreeMap<Symbol, Vec<Item>> = BTreeMap::new();
            for &item in &item_sets[states.len()] {
                if let Some(&symbol) = grammar.rules[item.rule].rhs.get(item.dot) {
                    transitions.entry(symbol).or_default().push(Item {
                        rule: item.rule,
                        dot: item.dot + 1,
                    });
                } else if item.rule == 0 {
                    state.accept = true;
                } else {
                    state.reductions.push(item.rule);
                }
            }
            for (symbol, seeds) in transitions {
                let items = Self::closure(&grammar, seeds);
                let target = *known.entry(items.clone()).or_insert_with(|| {
                    let id = item_sets.len();
                    item_sets.push(items);
                    id
                });
                match symbol {
                    Symbol::Terminal(id) => {
                        shifts.insert(id, target);
                        // Canonical LR states have one incoming grammar symbol.
                        // Preserve it independently of interval dispatch merging.
                        assert!(state.shift_symbols.insert(target, id).is_none());
                    }
                    Symbol::Nonterminal(id) => {
                        state.gotos.insert(id, target);
                    }
                }
            }
            state.dispatch = ShiftRange::build(&shifts, &grammar.terminals);
            #[cfg(test)]
            {
                state.shifts = shifts;
            }
            states.push(state);
        }
        Self {
            rules: grammar.rules,
            shapes: grammar.shapes,
            states,
        }
    }
}

#[cfg(test)]
mod optimization_tests;
