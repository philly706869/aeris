//! Recognition-only GLR execution using a graph-structured stack.
use std::collections::{BTreeMap, BTreeSet};

use crate::table::{Action, Table};

/// The first input position at which every GLR branch failed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ParseError {
    /// UTF-8 byte offset into the input (input.len() at end of input).
    pub offset: usize,
    /// Unexpected character, or None for unexpected end of input.
    pub found: Option<char>,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.found {
            Some(ch) => write!(f, "unexpected character {ch:?} at byte {}", self.offset),
            None => write!(f, "unexpected end of input at byte {}", self.offset),
        }
    }
}

impl std::error::Error for ParseError {}

struct Node {
    state: usize,
    predecessors: BTreeSet<usize>,
}

#[derive(Default)]
struct Stack {
    nodes: Vec<Node>,
}

impl Stack {
    // `layer` interns nodes by state at a single input position. Nodes from
    // earlier positions stay distinct even when they have the same LR state.
    fn node(&mut self, layer: &mut BTreeMap<usize, usize>, state: usize) -> usize {
        *layer.entry(state).or_insert_with(|| {
            let id = self.nodes.len();
            self.nodes.push(Node {
                state,
                predecessors: BTreeSet::new(),
            });
            id
        })
    }

    fn pop(&self, top: usize, count: usize) -> BTreeSet<usize> {
        let mut frontier = BTreeSet::from([top]);
        // Walk exactly the production length, including when nullable rules
        // create cycles in the graph. Never recursively enumerate whole stacks.
        for _ in 0..count {
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
}

impl Table {
    pub(crate) fn parse(&self, input: &str) -> Result<(), ParseError> {
        let mut stack = Stack::default();
        let mut active = BTreeMap::new();
        stack.node(&mut active, 0);

        for (offset, lookahead) in input
            .char_indices()
            .map(|(i, ch)| (i, Some(ch)))
            .chain(std::iter::once((input.len(), None)))
        {
            // Saturate reductions before shifting. A new predecessor edge may
            // enable paths for an already visited reduction, so revisit every
            // active node until the graph stops changing. At each position the
            // number of states and possible edges is finite, even for epsilon
            // cycles and infinitely ambiguous nullable grammars.
            loop {
                let mut changed = false;
                let tops: Vec<_> = active.values().copied().collect();
                for top in tops {
                    for action in self.actions(stack.nodes[top].state, lookahead) {
                        match action {
                            Action::Accept if lookahead.is_none() => return Ok(()),
                            Action::Reduce(id) => {
                                let rule = &self.productions[id];
                                for predecessor in stack.pop(top, rule.rhs.len()) {
                                    let state = stack.nodes[predecessor].state;
                                    let next = self.states[state].gotos[&rule.lhs];
                                    let target = stack.node(&mut active, next);
                                    changed |= stack.nodes[target].predecessors.insert(predecessor);
                                }
                            }
                            _ => {}
                        }
                    }
                }
                if !changed {
                    break;
                }
            }

            let mut next = BTreeMap::new();
            if lookahead.is_some() {
                for &top in active.values() {
                    for action in self.actions(stack.nodes[top].state, lookahead) {
                        if let Action::Shift(state) = action {
                            let target = stack.node(&mut next, state);
                            stack.nodes[target].predecessors.insert(top);
                        }
                    }
                }
            }
            if next.is_empty() {
                return Err(ParseError {
                    offset,
                    found: lookahead,
                });
            }
            active = next;
        }
        unreachable!("EOF must accept or fail")
    }
}

#[cfg(test)]
mod tests {
    use crate::{Cluster, ParseError, shard::*};

    macro_rules! root {
        ($name:ident, $data:expr) => {
            struct $name;
            impl Shard for $name {
                type Core = $name;
            }
            impl ShardCore for $name {
                type Output<'i> = ();
                const DATA: &'static ShardData = &$data;
            }
            impl StaticShard for $name {}
        };
    }

    #[test]
    fn nullable_cycles_and_unbounded_nullable_repetition() {
        struct Recursive;
        impl Shard for Recursive {
            type Core = Recursive;
        }
        impl ShardCore for Recursive {
            type Output<'i> = ();
            const DATA: &'static ShardData =
                &ShardData::option(&ShardData::reference::<Recursive>());
        }
        impl StaticShard for Recursive {}
        let cluster = Cluster::<Recursive>::build();
        assert_eq!(cluster.parse(""), Ok(()));
        assert!(cluster.parse("a").is_err());

        root!(
            Repeated,
            ShardData::vec(&ShardData::option(&ShardData::literal("a")), 0, usize::MAX)
        );
        let cluster = Cluster::<Repeated>::build();
        for input in ["", "a", "aa", "aaaa"] {
            assert_eq!(cluster.parse(input), Ok(()));
        }
        assert!(cluster.parse("aab").is_err());
    }

    #[test]
    fn recursive_nullable_binary_rule() {
        struct Recursive;
        impl Shard for Recursive {
            type Core = Recursive;
        }
        impl ShardCore for Recursive {
            type Output<'i> = ();
            const DATA: &'static ShardData = &ShardData::alternative(&[
                &ShardData::sequence(&[]),
                &ShardData::literal("a"),
                &ShardData::sequence(&[
                    &ShardData::reference::<Recursive>(),
                    &ShardData::reference::<Recursive>(),
                ]),
            ]);
        }
        impl StaticShard for Recursive {}
        let cluster = Cluster::<Recursive>::build();
        for input in ["", "a", "aa", "aaaa"] {
            assert_eq!(cluster.parse(input), Ok(()));
        }
        assert!(cluster.parse("aaaab").is_err());
    }

    #[test]
    fn nonproductive_cycle_rejects() {
        struct Recursive;
        impl Shard for Recursive {
            type Core = Recursive;
        }
        impl ShardCore for Recursive {
            type Output<'i> = ();
            const DATA: &'static ShardData = &ShardData::reference::<Recursive>();
        }
        impl StaticShard for Recursive {}
        let cluster = Cluster::<Recursive>::build();
        assert!(cluster.parse("").is_err());
        assert!(cluster.parse("a").is_err());
    }

    #[test]
    fn reports_byte_offsets_and_can_be_reused() {
        root!(Text, ShardData::literal("에🦀"));
        let cluster = Cluster::<Text>::build();
        assert_eq!(
            cluster.parse("에x"),
            Err(ParseError {
                offset: 3,
                found: Some('x')
            })
        );
        assert_eq!(
            cluster.parse("에"),
            Err(ParseError {
                offset: 3,
                found: None
            })
        );
        assert_eq!(
            cluster.parse("에🦀a"),
            Err(ParseError {
                offset: 7,
                found: Some('a')
            })
        );
        assert_eq!(cluster.parse("에🦀"), Ok(()));
        assert_eq!(
            cluster.parse(""),
            Err(ParseError {
                offset: 0,
                found: None
            })
        );
        assert_eq!(cluster.parse("에🦀"), Ok(()));
    }

    #[test]
    fn merged_branches_keep_distinct_predecessors() {
        // S -> a S a | a S | epsilon. Many different stacks reach the same
        // state at a position, and reductions must retain all predecessors.
        struct Recursive;
        impl Shard for Recursive {
            type Core = Recursive;
        }
        impl ShardCore for Recursive {
            type Output<'i> = ();
            const DATA: &'static ShardData = &ShardData::alternative(&[
                &ShardData::sequence(&[]),
                &ShardData::sequence(&[
                    &ShardData::literal("a"),
                    &ShardData::reference::<Recursive>(),
                    &ShardData::literal("a"),
                ]),
                &ShardData::sequence(&[
                    &ShardData::literal("a"),
                    &ShardData::reference::<Recursive>(),
                ]),
            ]);
        }
        impl StaticShard for Recursive {}
        let cluster = Cluster::<Recursive>::build();
        for n in 0..12 {
            assert_eq!(cluster.parse(&"a".repeat(n)), Ok(()));
        }
        assert!(cluster.parse("aaaaab").is_err());
    }
}
