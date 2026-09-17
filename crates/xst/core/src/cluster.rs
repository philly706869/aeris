use core::{any::TypeId, marker::PhantomData};
use rustc_hash::FxHashMap;
use std::collections::{BTreeMap, BTreeSet};

use crate::shard::{ReferenceData, SetData, ShardData, ShardDataKind, StaticShard};

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
        // Reserve production zero for the augmented start rule.
        let start = grammar.nonterminal();
        grammar.rule(start, vec![]);
        let root = grammar.reference(&ReferenceData::new::<S>());
        grammar.rules[0].rhs.push(Symbol::Nonterminal(root));
        Self {
            _shard: PhantomData,
            table: Table::build(grammar),
        }
    }

    pub fn parse(&self, input: &str) {
        todo!()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Symbol {
    Terminal(usize),
    Nonterminal(usize),
}

#[derive(Debug)]
enum Terminal {
    Char(char),
    Set(&'static SetData),
}

#[derive(Debug)]
struct Rule {
    lhs: usize,
    rhs: Vec<Symbol>,
}

#[derive(Default)]
struct Grammar {
    rules: Vec<Rule>,
    by_lhs: Vec<Vec<usize>>,
    terminals: Vec<Terminal>,
    chars: FxHashMap<char, usize>,
    references: FxHashMap<TypeId, usize>,
    nodes: FxHashMap<*const ShardData, usize>,
}

impl Grammar {
    fn nonterminal(&mut self) -> usize {
        let id = self.by_lhs.len();
        self.by_lhs.push(vec![]);
        id
    }

    fn rule(&mut self, lhs: usize, rhs: Vec<Symbol>) {
        self.by_lhs[lhs].push(self.rules.len());
        self.rules.push(Rule { lhs, rhs });
    }

    fn reference(&mut self, reference: &ReferenceData) -> usize {
        if let Some(&id) = self.references.get(&reference.id) {
            return id;
        }
        let id = self.nonterminal();
        // Register before descending so direct and mutual recursion terminate.
        self.references.insert(reference.id, id);
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
        match &data.kind {
            ShardDataKind::Literal(literal) => {
                let rhs = literal
                    .text
                    .chars()
                    .map(|ch| {
                        let terminal = *self.chars.entry(ch).or_insert_with(|| {
                            let index = self.terminals.len();
                            self.terminals.push(Terminal::Char(ch));
                            index
                        });
                        Symbol::Terminal(terminal)
                    })
                    .collect();
                self.rule(id, rhs);
            }
            ShardDataKind::Set(set) => {
                let terminal = self.terminals.len();
                self.terminals.push(Terminal::Set(set));
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
                // A chain keeps bounded repetition linear in the upper bound.
                let mut tail = id;
                for _ in 0..repeat.min {
                    let next = self.nonterminal();
                    self.rule(tail, vec![item, Symbol::Nonterminal(next)]);
                    tail = next;
                }
                if let Some(max) = repeat.max {
                    for _ in repeat.min..max {
                        let next = self.nonterminal();
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

    // Keep preference in production order without discarding GLR alternatives.
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
    // All matching terminal predicates must be considered, including overlaps.
    shifts: BTreeMap<usize, usize>,
    gotos: BTreeMap<usize, usize>,
    // LR(0) reductions apply on every lookahead, including EOF. Multiple
    // reductions and shifts coexist rather than resolving conflicts.
    reductions: Vec<usize>,
    // Only the completed augmented start rule accepts, and only at EOF.
    accept: bool,
}

#[derive(Debug)]
struct Table {
    rules: Vec<Rule>,
    terminals: Vec<Terminal>,
    states: Vec<State>,
}

impl Table {
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
        let initial = Self::closure(&grammar, [Item { rule: 0, dot: 0 }]);
        let mut known = FxHashMap::default();
        known.insert(initial.clone(), 0);
        let mut item_sets = vec![initial];
        let mut states = Vec::new();
        while states.len() < item_sets.len() {
            let mut state = State::default();
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
                        state.shifts.insert(id, target);
                    }
                    Symbol::Nonterminal(id) => {
                        state.gotos.insert(id, target);
                    }
                }
            }
            states.push(state);
        }
        Self {
            rules: grammar.rules,
            terminals: grammar.terminals,
            states,
        }
    }
}

#[cfg(test)]
mod tests;
