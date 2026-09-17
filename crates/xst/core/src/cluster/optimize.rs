use std::collections::BTreeMap;

use super::{CharSet, Grammar, Item, Symbol};

#[derive(Clone, Debug, Default)]
pub struct First {
    pub chars: CharSet,
    pub nullable: bool,
}

impl First {
    pub fn sequence(grammar: &Grammar, first: &[Self], symbols: &[Symbol]) -> Self {
        let mut result = Self {
            nullable: true,
            ..Self::default()
        };
        for symbol in symbols {
            match *symbol {
                Symbol::Terminal(id) => {
                    result.chars.union(&grammar.terminals[id]);
                    result.nullable = false;
                }
                Symbol::Nonterminal(id) => {
                    result.chars.union(&first[id].chars);
                    result.nullable = first[id].nullable;
                }
            }
            if !result.nullable {
                break;
            }
        }
        result
    }

    pub fn build(grammar: &Grammar) -> Vec<Self> {
        let mut first = vec![Self::default(); grammar.by_lhs.len()];
        loop {
            let mut changed = false;
            for rule in &grammar.rules {
                let value = Self::sequence(grammar, &first, &rule.rhs);
                changed |= first[rule.lhs].chars.union(&value.chars);
                if value.nullable && !first[rule.lhs].nullable {
                    first[rule.lhs].nullable = true;
                    changed = true;
                }
            }
            if !changed {
                return first;
            }
        }
    }

    pub fn state(grammar: &Grammar, first: &[Self], items: &[Item]) -> Self {
        let mut result = Self::default();
        for item in items {
            let suffix = Self::sequence(grammar, first, &grammar.rules[item.rule].rhs[item.dot..]);
            result.chars.union(&suffix.chars);
            result.nullable |= suffix.nullable;
        }
        result
    }

    pub fn permits(&self, lookahead: Option<char>) -> bool {
        // A nullable suffix may reduce into a caller accepting this lookahead.
        // FIRST alone cannot reject it; no FOLLOW/SLR conflict filtering here.
        self.nullable || lookahead.is_some_and(|ch| self.chars.matches(ch))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ShiftRange {
    pub lo: u32,
    pub hi: u32,
    pub targets: Vec<usize>,
}

impl ShiftRange {
    pub fn build(shifts: &BTreeMap<usize, usize>, terminals: &[CharSet]) -> Vec<Self> {
        let mut events: BTreeMap<u32, Vec<(usize, i64)>> = BTreeMap::new();
        for (&terminal, &target) in shifts {
            for &(lo, hi) in &terminals[terminal].0 {
                events.entry(lo).or_default().push((target, 1));
                events.entry(hi + 1).or_default().push((target, -1));
            }
        }
        let mut active = BTreeMap::<usize, i64>::new();
        let mut previous = 0;
        let mut ranges: Vec<Self> = Vec::new();
        for (boundary, changes) in events {
            if previous < boundary && !active.is_empty() {
                let targets: Vec<_> = active.keys().copied().collect();
                if let Some(last) = ranges
                    .last_mut()
                    .filter(|last| last.hi + 1 == previous && last.targets == targets)
                {
                    last.hi = boundary - 1;
                } else {
                    ranges.push(Self {
                        lo: previous,
                        hi: boundary - 1,
                        targets,
                    });
                }
            }
            for (target, delta) in changes {
                let count = active.entry(target).or_default();
                *count += delta;
                if *count == 0 {
                    active.remove(&target);
                }
            }
            previous = boundary;
        }
        ranges
    }

    pub fn lookup(ranges: &[Self], ch: char) -> &[usize] {
        let ch = ch as u32;
        let index = ranges.partition_point(|range| range.hi < ch);
        ranges
            .get(index)
            .filter(|range| range.lo <= ch)
            .map_or(&[], |range| range.targets.as_slice())
    }
}
