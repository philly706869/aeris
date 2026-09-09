//! Unoptimized SLR tables. Conflicts are retained for the GLR executor.
use core::{any::TypeId, ops::RangeInclusive};
use std::collections::{BTreeMap, BTreeSet, HashMap};

use crate::shard::ShardData;

type Nonterminal = usize;
type Terminal = usize;
type StateId = usize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Symbol {
    Terminal(Terminal),
    Nonterminal(Nonterminal),
}

#[derive(Debug)]
pub(crate) struct Production {
    pub lhs: Nonterminal,
    pub rhs: Vec<Symbol>,
}

/// A terminal is a predicate on Unicode scalar values. Overlapping predicates
/// remain separate: an executor must collect actions for *all* matching ones.
#[derive(Debug)]
pub(crate) enum Character {
    Literal(char),
    Set {
        negated: bool,
        ranges: &'static [RangeInclusive<char>],
    },
}

impl Character {
    pub fn matches(&self, ch: char) -> bool {
        match self {
            Self::Literal(value) => *value == ch,
            Self::Set { negated, ranges } => ranges.iter().any(|r| r.contains(&ch)) != *negated,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Action {
    Shift(StateId),
    Reduce(usize),
    Accept,
}

#[derive(Debug, Default)]
pub(crate) struct State {
    /// None denotes end of input; Some(t) denotes a character predicate.
    pub actions: BTreeMap<Option<Terminal>, Vec<Action>>,
    pub gotos: BTreeMap<Nonterminal, StateId>,
}

#[derive(Debug)]
pub(crate) struct Table {
    pub characters: Vec<Character>,
    pub productions: Vec<Production>,
    /// State zero is the initial state. Production zero is the augmented root.
    pub states: Vec<State>,
}

#[derive(Default)]
struct Grammar {
    characters: Vec<Character>,
    productions: Vec<Production>,
    nonterminals: usize,
    externs: HashMap<TypeId, Nonterminal>,
}

impl Grammar {
    fn nonterminal(&mut self) -> Nonterminal {
        let id = self.nonterminals;
        self.nonterminals += 1;
        id
    }

    fn rule(&mut self, lhs: Nonterminal, rhs: Vec<Symbol>) {
        self.productions.push(Production { lhs, rhs });
    }

    fn terminal(&mut self, character: Character) -> Symbol {
        let id = self.characters.len();
        self.characters.push(character);
        Symbol::Terminal(id)
    }

    fn lower(&mut self, data: &'static ShardData) -> Nonterminal {
        if let ShardData::Extern(data) = data {
            if let Some(&id) = self.externs.get(&data.id) {
                return id;
            }
            let id = self.nonterminal();
            // Register before following the reference to terminate recursive grammars.
            self.externs.insert(data.id, id);
            self.define(id, (data.reference)());
            return id;
        }
        let id = self.nonterminal();
        self.define(id, data);
        id
    }

    fn define(&mut self, lhs: Nonterminal, data: &'static ShardData) {
        use Symbol::Nonterminal as N;
        match data {
            ShardData::Literal(data) => {
                let rhs = data
                    .text
                    .chars()
                    .map(|ch| self.terminal(Character::Literal(ch)))
                    .collect();
                self.rule(lhs, rhs);
            }
            ShardData::Set(data) => {
                let symbol = self.terminal(Character::Set {
                    negated: data.negated,
                    ranges: data.range,
                });
                self.rule(lhs, vec![symbol]);
            }
            ShardData::Option(data) => {
                let item = self.lower(data.item);
                self.rule(lhs, vec![]);
                self.rule(lhs, vec![N(item)]);
            }
            ShardData::Vec(data) => {
                assert!(
                    data.min <= data.max,
                    "shard repetition minimum exceeds maximum"
                );
                let item = self.lower(data.item);
                let mut current = lhs;
                // A chain avoids duplicating the mandatory prefix for each length.
                for _ in 0..data.min {
                    let next = self.nonterminal();
                    self.rule(current, vec![N(item), N(next)]);
                    current = next;
                }
                if data.max == usize::MAX {
                    self.rule(current, vec![]);
                    self.rule(current, vec![N(item), N(current)]);
                } else {
                    for _ in data.min..data.max {
                        self.rule(current, vec![]);
                        let next = self.nonterminal();
                        self.rule(current, vec![N(item), N(next)]);
                        current = next;
                    }
                    self.rule(current, vec![]);
                }
            }
            ShardData::Sequence(data) => {
                let rhs = data.items.iter().map(|item| N(self.lower(item))).collect();
                self.rule(lhs, rhs);
            }
            ShardData::Alternative(data) => {
                for item in data.items {
                    let item = self.lower(item);
                    self.rule(lhs, vec![N(item)]);
                }
            }
            ShardData::Extern(_) => {
                let item = self.lower(data);
                self.rule(lhs, vec![N(item)]);
            }
        }
    }

    fn follow(&self, root: Nonterminal) -> Vec<BTreeSet<Option<Terminal>>> {
        let mut nullable = vec![false; self.nonterminals];
        let mut first = vec![BTreeSet::new(); self.nonterminals];
        loop {
            let mut changed = false;
            for rule in &self.productions {
                let mut empty = true;
                for symbol in &rule.rhs {
                    match *symbol {
                        Symbol::Terminal(t) => {
                            changed |= first[rule.lhs].insert(t);
                            empty = false;
                            break;
                        }
                        Symbol::Nonterminal(n) => {
                            for t in first[n].clone() {
                                changed |= first[rule.lhs].insert(t);
                            }
                            if !nullable[n] {
                                empty = false;
                                break;
                            }
                        }
                    }
                }
                if empty && !nullable[rule.lhs] {
                    nullable[rule.lhs] = true;
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }
        let mut follow = vec![BTreeSet::new(); self.nonterminals];
        follow[root].insert(None);
        loop {
            let mut changed = false;
            for rule in &self.productions {
                let mut suffix = follow[rule.lhs].clone();
                for symbol in rule.rhs.iter().rev() {
                    match *symbol {
                        Symbol::Terminal(t) => suffix = BTreeSet::from([Some(t)]),
                        Symbol::Nonterminal(n) => {
                            for t in &suffix {
                                changed |= follow[n].insert(*t);
                            }
                            if !nullable[n] {
                                suffix.clear();
                            }
                            suffix.extend(first[n].iter().copied().map(Some));
                        }
                    }
                }
            }
            if !changed {
                break;
            }
        }
        follow
    }

    fn closure(&self, mut items: BTreeSet<(usize, usize)>) -> BTreeSet<(usize, usize)> {
        loop {
            let mut added = Vec::new();
            for &(rule, dot) in &items {
                if let Some(Symbol::Nonterminal(n)) = self.productions[rule].rhs.get(dot) {
                    for (id, rule) in self.productions.iter().enumerate() {
                        if rule.lhs == *n && !items.contains(&(id, 0)) {
                            added.push((id, 0));
                        }
                    }
                }
            }
            if added.is_empty() {
                return items;
            }
            items.extend(added);
        }
    }
}

impl Table {
    pub fn build(root_id: TypeId, data: &'static ShardData) -> Self {
        let mut grammar = Grammar::default();
        let augmented = grammar.nonterminal();
        let root = grammar.nonterminal();
        grammar.externs.insert(root_id, root);
        grammar.rule(augmented, vec![Symbol::Nonterminal(root)]);
        grammar.define(root, data);
        let follow = grammar.follow(augmented);
        let initial = grammar.closure(BTreeSet::from([(0, 0)]));
        let mut ids = BTreeMap::from([(initial.clone(), 0)]);
        let mut item_sets = vec![initial];
        let mut states = Vec::new();
        while states.len() < item_sets.len() {
            let items = &item_sets[states.len()];
            let mut state = State::default();
            let mut transitions: BTreeMap<Symbol, BTreeSet<(usize, usize)>> = BTreeMap::new();
            for &(id, dot) in items {
                let rule = &grammar.productions[id];
                if let Some(&symbol) = rule.rhs.get(dot) {
                    transitions.entry(symbol).or_default().insert((id, dot + 1));
                } else if id == 0 {
                    state.actions.entry(None).or_default().push(Action::Accept);
                } else {
                    for &lookahead in &follow[rule.lhs] {
                        state
                            .actions
                            .entry(lookahead)
                            .or_default()
                            .push(Action::Reduce(id));
                    }
                }
            }
            for (symbol, kernel) in transitions {
                let target = grammar.closure(kernel);
                let next = if let Some(&id) = ids.get(&target) {
                    id
                } else {
                    let id = item_sets.len();
                    ids.insert(target.clone(), id);
                    item_sets.push(target);
                    id
                };
                match symbol {
                    Symbol::Terminal(t) => state
                        .actions
                        .entry(Some(t))
                        .or_default()
                        .push(Action::Shift(next)),
                    Symbol::Nonterminal(n) => {
                        state.gotos.insert(n, next);
                    }
                }
            }
            states.push(state);
        }
        Self {
            characters: grammar.characters,
            productions: grammar.productions,
            states,
        }
    }

    /// Union actions across overlapping predicates, retaining every GLR branch.
    pub fn actions(&self, state: StateId, lookahead: Option<char>) -> Vec<Action> {
        let mut actions = BTreeSet::new();
        for (&terminal, entries) in &self.states[state].actions {
            let matches = match (terminal, lookahead) {
                (None, None) => true,
                (Some(t), Some(ch)) => self.characters[t].matches(ch),
                _ => false,
            };
            if matches {
                actions.extend(entries);
            }
        }
        actions.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shard::*;

    struct A;
    impl ShardLiteral for A {
        const LITERAL: &'static str = "a";
    }
    struct Unicode;
    impl ShardLiteral for Unicode {
        const LITERAL: &'static str = "에🦀";
    }
    type Lit = LiteralType<A>;

    fn table<T: ShardDataType>() -> Table {
        Table::build(TypeId::of::<T>(), T::DATA)
    }

    // Small exhaustive stack interpreter, independent of the future GLR runtime.
    // Tests use grammars without nullable recursive cycles.
    fn accepts(table: &Table, input: &str) -> bool {
        let chars: Vec<_> = input.chars().collect();
        let mut pending = vec![(vec![0], 0)];
        let mut seen = BTreeSet::new();
        while let Some((stack, offset)) = pending.pop() {
            if !seen.insert((stack.clone(), offset)) {
                continue;
            }
            assert!(seen.len() < 100_000, "test interpreter did not converge");
            for action in table.actions(*stack.last().unwrap(), chars.get(offset).copied()) {
                let mut stack = stack.clone();
                match action {
                    Action::Accept => return true,
                    Action::Shift(next) => {
                        stack.push(next);
                        pending.push((stack, offset + 1));
                    }
                    Action::Reduce(id) => {
                        let rule = &table.productions[id];
                        assert!(stack.len() > rule.rhs.len());
                        stack.truncate(stack.len() - rule.rhs.len());
                        let next = table.states[*stack.last().unwrap()].gotos[&rule.lhs];
                        stack.push(next);
                        pending.push((stack, offset));
                    }
                }
            }
        }
        false
    }

    fn check<T: ShardDataType>(yes: &[&str], no: &[&str]) {
        let table = table::<T>();
        for input in yes {
            assert!(accepts(&table, input), "rejected {input:?}");
            assert!(table.parse(input).is_ok(), "GLR rejected {input:?}");
        }
        for input in no {
            assert!(!accepts(&table, input), "accepted {input:?}");
            assert!(table.parse(input).is_err(), "GLR accepted {input:?}");
        }
    }

    #[test]
    fn literals_and_empty_grammars() {
        check::<LiteralType<Unicode>>(&["에🦀"], &["", "에", "에🦀a"]);
        check::<SequenceType<()>>(&[""], &["a"]);
        check::<AlternativeType<()>>(&[], &["", "a"]);
    }

    #[test]
    fn nullable_prefix_and_suffix() {
        type Grammar = SequenceType<(OptionType<Lit>, Lit, OptionType<Lit>)>;
        check::<Grammar>(&["a", "aa", "aaa"], &["", "aaaa", "b"]);
    }

    #[test]
    fn repetitions() {
        check::<VecType<Lit, 2, 4>>(&["aa", "aaa", "aaaa"], &["", "a", "aaaaa"]);
        check::<VecType<Lit, 0, 0>>(&[""], &["a"]);
        check::<VecType<Lit, 2, 2>>(&["aa"], &["", "a", "aaa"]);
        check::<VecType<Lit, 0, { usize::MAX }>>(&["", "a", "aaaaa"], &["b", "ab"]);
        check::<VecType<Lit, 2, { usize::MAX }>>(&["aa", "aaaaa"], &["", "a"]);
        check::<VecType<OptionType<Lit>, 1, 3>>(&["", "a", "aaa"], &["aaaa"]);
    }

    struct Letters;
    impl ShardSet for Letters {
        const SET: &'static [std::ops::RangeInclusive<char>] = &['a'..='z', 'b'..='d'];
    }

    #[test]
    fn overlapping_and_negated_character_sets() {
        type Grammar = AlternativeType<(Lit, SetType<false, Letters>)>;
        let table = table::<Grammar>();
        assert_eq!(
            table
                .actions(0, Some('a'))
                .iter()
                .filter(|a| matches!(a, Action::Shift(_)))
                .count(),
            2
        );
        check::<Grammar>(&["a", "b", "z"], &["", "aa", "에"]);
        check::<SetType<true, Letters>>(&["에", "🦀", "\0"], &["", "a", "z", "AA"]);
    }

    struct Recursive;
    impl Shard for Recursive {
        type Data = AlternativeType<(
            Lit,
            SequenceType<(ExternType<Recursive>, ExternType<Recursive>)>,
        )>;
    }
    impl StaticShard for Recursive {}

    #[test]
    fn recursive_root_and_shift_reduce_conflicts() {
        let cluster = crate::Cluster::<Recursive>::build();
        let table = &cluster.table;
        assert!(table.states.iter().enumerate().any(|(id, _)| {
            let actions = table.actions(id, Some('a'));
            actions.iter().any(|a| matches!(a, Action::Shift(_)))
                && actions.iter().any(|a| matches!(a, Action::Reduce(_)))
        }));
        for text in ["a", "aa", "aaa", "aaaa"] {
            assert!(accepts(table, text));
            assert_eq!(cluster.parse(text), Ok(()));
        }
        for text in ["", "b", "aab"] {
            assert!(!accepts(table, text));
            assert!(cluster.parse(text).is_err());
        }
    }

    #[test]
    fn reduce_reduce_conflicts() {
        type Grammar = AlternativeType<(SequenceType<()>, OptionType<Lit>)>;
        let table = table::<Grammar>();
        assert!(
            table
                .actions(0, None)
                .iter()
                .filter(|a| matches!(a, Action::Reduce(_)))
                .count()
                >= 2
        );
        check::<Grammar>(&["", "a"], &["aa"]);
    }

    struct Left;
    struct Right;
    impl Shard for Left {
        type Data = AlternativeType<(Lit, SequenceType<(Lit, ExternType<Right>)>)>;
    }
    impl Shard for Right {
        type Data = ExternType<Left>;
    }

    #[test]
    fn mutual_recursion() {
        check::<<Left as Shard>::Data>(&["a", "aa", "aaa"], &["", "b"]);
    }

    #[test]
    fn glr_matches_exhaustive_stack_interpreter() {
        struct B;
        impl ShardLiteral for B {
            const LITERAL: &'static str = "b";
        }
        type Grammar = AlternativeType<(
            SequenceType<(VecType<Lit, 0, 4>, LiteralType<B>)>,
            SequenceType<(Lit, VecType<AlternativeType<(Lit, LiteralType<B>)>, 0, 3>)>,
            SequenceType<(OptionType<Lit>, OptionType<Lit>)>,
        )>;
        let table = table::<Grammar>();
        for len in 0..=6 {
            for bits in 0..(1 << len) {
                let input: String = (0..len)
                    .map(|i| if bits & (1 << i) == 0 { 'a' } else { 'b' })
                    .collect();
                assert_eq!(
                    table.parse(&input).is_ok(),
                    accepts(&table, &input),
                    "{input:?}"
                );
            }
        }
    }

    #[test]
    fn nullable_recursive_grammar_builds() {
        struct EmptyRecursive;
        impl Shard for EmptyRecursive {
            type Data = OptionType<ExternType<EmptyRecursive>>;
        }
        assert!(!table::<<EmptyRecursive as Shard>::Data>().states.is_empty());
    }

    #[test]
    #[should_panic(expected = "shard repetition minimum exceeds maximum")]
    fn invalid_repetition_is_rejected() {
        table::<VecType<Lit, 2, 1>>();
    }
}
