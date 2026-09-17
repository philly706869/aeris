use super::*;
use crate::shard::{Shard, ShardCore};

fn table(data: &'static ShardData) -> Table {
    let mut grammar = Grammar::default();
    let start = grammar.nonterminal();
    grammar.rule(start, vec![]);
    let root = grammar.lower(data);
    grammar.rules[0].rhs.push(Symbol::Nonterminal(root));
    Table::build(grammar)
}

// A bounded, test-only nondeterministic recognizer exercises actual table
// actions, including reductions before EOF and overlapping terminal predicates.
fn accepts(table: &Table, input: &str) -> bool {
    let chars: Vec<_> = input.chars().collect();
    let mut pending = vec![(vec![0], 0)];
    let mut seen = BTreeSet::new();
    while let Some((stack, offset)) = pending.pop() {
        if !seen.insert((stack.clone(), offset)) {
            continue;
        }
        assert!(seen.len() < 100_000, "unexpected unbounded test grammar");
        let state = &table.states[*stack.last().unwrap()];
        if offset == chars.len() && state.accept {
            return true;
        }
        if let Some(&ch) = chars.get(offset) {
            for (&terminal, &target) in &state.shifts {
                let matches = match &table.terminals[terminal] {
                    Terminal::Char(expected) => ch == *expected,
                    Terminal::Set(set) => {
                        set.range.iter().any(|range| range.contains(&ch)) != set.negated
                    }
                };
                if matches {
                    let mut next = stack.clone();
                    next.push(target);
                    pending.push((next, offset + 1));
                }
            }
        }
        for &rule in &state.reductions {
            let rule = &table.rules[rule];
            if stack.len() <= rule.rhs.len() {
                continue;
            }
            let mut next = stack[..stack.len() - rule.rhs.len()].to_vec();
            if let Some(&target) = table.states[*next.last().unwrap()].gotos.get(&rule.lhs) {
                next.push(target);
                pending.push((next, offset));
            }
        }
    }
    false
}

#[test]
fn literals_sequences_and_eof() {
    let t = table(
        const {
            const DATA: &'static ShardData =
                &ShardData::sequence(&[&ShardData::literal(""), &ShardData::literal("한🦀")]);
            DATA
        },
    );
    assert!(accepts(&t, "한🦀"));
    for input in ["", "한", "한🦀x", "x한🦀"] {
        assert!(!accepts(&t, input));
    }
    assert_eq!(t.states.iter().filter(|state| state.accept).count(), 1);
    assert!(accepts(
        &table(
            const {
                const DATA: &'static ShardData = &ShardData::sequence(&[]);
                DATA
            }
        ),
        ""
    ));
    assert!(!accepts(
        &table(
            const {
                const DATA: &'static ShardData = &ShardData::alternative(&[]);
                DATA
            }
        ),
        ""
    ));
}

#[test]
fn repetitions_and_options() {
    for data in [
        const {
            const DATA: &'static ShardData = &ShardData::vec(&ShardData::literal("a"), 2, Some(4));
            DATA
        },
        const {
            const DATA: &'static ShardData =
                &ShardData::vec_lazy(&ShardData::literal("a"), 2, Some(4));
            DATA
        },
    ] {
        let t = table(data);
        for n in 0..7 {
            assert_eq!(accepts(&t, &"a".repeat(n)), (2..=4).contains(&n));
        }
    }
    let exact = table(
        const {
            const DATA: &'static ShardData = &ShardData::vec(&ShardData::literal("a"), 3, Some(3));
            DATA
        },
    );
    let zero = table(
        const {
            const DATA: &'static ShardData = &ShardData::vec(&ShardData::literal("a"), 0, Some(0));
            DATA
        },
    );
    let star = table(
        const {
            const DATA: &'static ShardData = &ShardData::vec(&ShardData::literal("a"), 0, None);
            DATA
        },
    );
    let plus = table(
        const {
            const DATA: &'static ShardData =
                &ShardData::vec_lazy(&ShardData::literal("a"), 1, None);
            DATA
        },
    );
    let option = table(
        const {
            const DATA: &'static ShardData = &ShardData::option(&ShardData::literal("a"));
            DATA
        },
    );
    let lazy = table(
        const {
            const DATA: &'static ShardData = &ShardData::option_lazy(&ShardData::literal("a"));
            DATA
        },
    );
    for n in 0..7 {
        let input = "a".repeat(n);
        assert_eq!(accepts(&exact, &input), n == 3);
        assert_eq!(accepts(&zero, &input), n == 0);
        assert!(accepts(&star, &input));
        assert_eq!(accepts(&plus, &input), n > 0);
        assert_eq!(accepts(&option, &input), n <= 1);
        assert_eq!(accepts(&lazy, &input), n <= 1);
    }
}

#[test]
fn overlapping_and_negated_sets_keep_all_branches() {
    let t = table(
        const {
            const DATA: &'static ShardData = &ShardData::alternative(&[
                &ShardData::sequence(&[
                    &ShardData::set(false, &['a'..='z']),
                    &ShardData::literal("x"),
                ]),
                &ShardData::sequence(&[&ShardData::literal("a"), &ShardData::literal("y")]),
                &ShardData::sequence(&[
                    &ShardData::set(true, &['0'..='9']),
                    &ShardData::literal("z"),
                ]),
            ]);
            DATA
        },
    );
    for input in ["ax", "ay", "az", "🦀z"] {
        assert!(accepts(&t, input));
    }
    for input in ["1z", "by", "a"] {
        assert!(!accepts(&t, input));
    }
    assert_eq!(t.states[0].shifts.len(), 3);
}

struct Recursive;
impl Shard for Recursive {
    type Core = Self;
}
impl StaticShard for Recursive {}
impl ShardCore for Recursive {
    type Output<'i> = ();
    const DATA: &'static ShardData = const {
        const DATA: &'static ShardData = &ShardData::alternative(&[
            &ShardData::sequence(&[
                &ShardData::reference::<Recursive>(),
                &ShardData::reference::<Recursive>(),
            ]),
            &ShardData::literal("a"),
        ]);
        DATA
    };
}

#[test]
fn recursive_ambiguous_grammar_preserves_conflicts() {
    let cluster = Cluster::<Recursive>::build();
    assert!(
        cluster
            .table
            .states
            .iter()
            .any(|state| !state.shifts.is_empty() && !state.reductions.is_empty())
    );
    for input in ["a", "aa", "aaa", "aaaa"] {
        assert!(accepts(&cluster.table, input));
    }
    assert!(!accepts(&cluster.table, ""));
    assert!(!accepts(&cluster.table, "ab"));
    let t = table(
        const {
            const DATA: &'static ShardData =
                &ShardData::alternative(&[&ShardData::sequence(&[]), &ShardData::literal("")]);
            DATA
        },
    );
    assert!(t.states[0].reductions.len() >= 2);
    assert!(accepts(&t, ""));
    // Canonical item sets and stable traversal produce reproducible tables.
    assert_eq!(
        format!("{:?}", cluster.table),
        format!("{:?}", Cluster::<Recursive>::build().table)
    );
}

#[test]
fn nullable_recursive_repetition_builds_finitely() {
    let t = table(
        const {
            const DATA: &'static ShardData = &ShardData::vec(&ShardData::literal(""), 0, None);
            DATA
        },
    );
    assert!(t.states.len() < 20);
    assert!(t.states.iter().any(|state| state.accept));
}

#[test]
#[should_panic(expected = "invalid repetition bounds")]
fn invalid_bounds_are_rejected() {
    table(
        const {
            const DATA: &'static ShardData = &ShardData::vec(&ShardData::literal("a"), 2, Some(1));
            DATA
        },
    );
}
