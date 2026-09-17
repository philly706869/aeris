use super::*;
use crate::shard::SetData;

#[test]
fn sets_are_canonical_over_unicode_scalars() {
    let data = SetData::new(false, &['d'..='f', 'a'..='c', 'b'..='e', 'z'..='a']);
    let set = CharSet::from_data(&data);
    assert_eq!(set, CharSet::new([('a' as u32, 'f' as u32)]));
    let negated = CharSet::from_data(&SetData::new(true, &['b'..='d']));
    let universal = CharSet::from_data(&SetData::new(true, &[]));
    assert_eq!(universal.0, vec![(0, 0xd7ff), (0xe000, 0x10ffff)]);
    for ch in (0..=0x10ffff).filter_map(char::from_u32) {
        assert_eq!(set.matches(ch), data.range.iter().any(|r| r.contains(&ch)));
        assert_eq!(negated.matches(ch), !('b'..='d').contains(&ch));
        assert!(universal.matches(ch));
    }
    assert!(
        CharSet::from_data(&SetData::new(true, &['\0'..='\u{10ffff}']))
            .0
            .is_empty()
    );
}

#[test]
fn equivalent_literals_and_sets_share_terminals() {
    const DATA: &ShardData = &ShardData::alternative(&[
        &ShardData::literal("a"),
        &ShardData::set(false, &['a'..='a', 'a'..='a']),
        &ShardData::set(true, &['\0'..='`', 'b'..='\u{10ffff}']),
    ]);
    let mut grammar = Grammar::default();
    grammar.lower(DATA);
    assert_eq!(grammar.terminals.len(), 1);
}

#[test]
fn first_merges_recursive_nullable_prefixes() {
    let mut grammar = Grammar::default();
    let a = grammar.nonterminal();
    let b = grammar.nonterminal();
    let c = grammar.nonterminal();
    let x = grammar.terminal(CharSet::new([(120, 120)]));
    let y = grammar.terminal(CharSet::new([(121, 121)]));
    grammar.rule(a, vec![Symbol::Nonterminal(b), Symbol::Terminal(y)]);
    grammar.rule(b, vec![Symbol::Nonterminal(c)]);
    grammar.rule(c, vec![Symbol::Nonterminal(b)]);
    grammar.rule(c, vec![]);
    grammar.rule(c, vec![Symbol::Terminal(x)]);
    let first = First::build(&grammar);
    assert_eq!(first[a].chars.0, vec![(120, 121)]);
    assert!(!first[a].nullable);
    assert!(first[b].nullable && first[c].nullable);
    assert_eq!(first[b].chars.0, vec![(120, 120)]);
}

#[test]
fn dispatch_preserves_overlaps_and_merges_identical_destinations() {
    let terminals = vec![
        CharSet::new([(0, 100)]),
        CharSet::new([(50, 150)]),
        CharSet::new([(151, 200)]),
        CharSet::from_data(&SetData::new(true, &['\0'..='z'])),
    ];
    let shifts = BTreeMap::from([(0, 1), (1, 1), (2, 1), (3, 2)]);
    let ranges = ShiftRange::build(&shifts, &terminals);
    assert_eq!(
        ranges[0],
        ShiftRange {
            lo: 0,
            hi: 122,
            targets: vec![1]
        }
    );
    for ch in (0..=0x10ffff).filter_map(char::from_u32) {
        let expected: BTreeSet<_> = shifts
            .iter()
            .filter(|(id, _)| terminals[**id].matches(ch))
            .map(|(_, target)| *target)
            .collect();
        assert_eq!(
            ShiftRange::lookup(&ranges, ch),
            expected.into_iter().collect::<Vec<_>>()
        );
    }
}

// Independent explicit-stack LR(0) recognizer: no FIRST pruning or interval
// dispatch. Use a finite, non-cyclic grammar to bound stack enumeration.
fn reference_parse(table: &Table, terminals: &[CharSet], input: &str) -> bool {
    let chars: Vec<_> = input.chars().collect();
    let mut pending = vec![(vec![0], 0)];
    let mut seen = BTreeSet::new();
    while let Some((stack, offset)) = pending.pop() {
        if !seen.insert((stack.clone(), offset)) {
            continue;
        }
        let state = &table.states[*stack.last().unwrap()];
        if state.accept && offset == chars.len() {
            return true;
        }
        for &id in &state.reductions {
            let rule = &table.rules[id];
            if stack.len() <= rule.rhs.len() {
                continue;
            }
            let mut next = stack[..stack.len() - rule.rhs.len()].to_vec();
            if let Some(&target) = table.states[*next.last().unwrap()].gotos.get(&rule.lhs) {
                next.push(target);
                pending.push((next, offset));
            }
        }
        if let Some(&ch) = chars.get(offset) {
            for (&id, &target) in &state.shifts {
                if terminals[id].matches(ch) {
                    let mut next = stack.clone();
                    next.push(target);
                    pending.push((next, offset + 1));
                }
            }
        }
    }
    false
}

#[test]
fn optimized_parser_agrees_with_unfiltered_lr0() {
    const DATA: &ShardData = &ShardData::alternative(&[
        &ShardData::sequence(&[
            &ShardData::option(&ShardData::literal("a")),
            &ShardData::set(false, &['a'..='b']),
            &ShardData::vec(&ShardData::literal("b"), 0, Some(2)),
        ]),
        &ShardData::sequence(&[
            &ShardData::set(true, &['b'..='z']),
            &ShardData::literal("b"),
        ]),
        &ShardData::literal(""),
    ]);
    let mut grammar = Grammar::default();
    let start = grammar.nonterminal();
    grammar.rule(start, vec![]);
    let root = grammar.lower(DATA);
    grammar.rules[0].rhs.push(Symbol::Nonterminal(root));
    let terminals = grammar.terminals.clone();
    let table = Table::build(grammar);
    assert!(
        table
            .states
            .iter()
            .any(|state| !state.first.permits(Some('z')))
    );
    let mut words = vec![String::new()];
    for _ in 0..5 {
        let mut next = Vec::new();
        for word in &words {
            assert_eq!(
                table.parse(word),
                reference_parse(&table, &terminals, word),
                "{word:?}"
            );
            for ch in ['a', 'b', 'z', '🦀'] {
                next.push(format!("{word}{ch}"));
            }
        }
        words = next;
    }
}

#[test]
fn optimization_preserves_derivation_structure_and_spans() {
    use crate::shard::{Shard, ShardCore};
    struct Root;
    impl Shard for Root {
        type Core = Self;
    }
    impl StaticShard for Root {}
    impl ShardCore for Root {
        type Output<'i> = Vec<(usize, usize, usize, usize)>;
        const DATA: &'static ShardData = &ShardData::alternative(&[
            &ShardData::sequence(&[
                &ShardData::option(&ShardData::literal("a")),
                &ShardData::set(false, &['a'..='b', 'a'..='a']),
                &ShardData::vec(&ShardData::literal("b"), 0, Some(2)),
            ]),
            &ShardData::sequence(&[
                &ShardData::set(true, &['b'..='z']),
                &ShardData::literal("b"),
            ]),
            &ShardData::literal("a"),
            &ShardData::set(false, &['a'..='a']),
            &ShardData::literal(""),
        ]);
        fn map<'a, 'i: 'a>(node: MappingNode<'a, 'i>) -> MappingTask<'a, Self::Output<'i>> {
            MappingTask::ready(Ok(node
                .nodes
                .iter()
                .filter_map(|node| {
                    let Some(Symbol::Nonterminal(id)) = node.symbol else {
                        return None;
                    };
                    Some((id, node.rule.unwrap(), node.start, node.end))
                })
                .collect()))
        }
    }
    let optimized = Cluster::<Root>::build();
    let mut grammar = Grammar {
        unshared_terminals: true,
        ..Grammar::default()
    };
    let start = grammar.nonterminal();
    grammar.rule(start, vec![]);
    let root = grammar.reference(&ReferenceData::new::<Root>());
    grammar.rules[0].rhs.push(Symbol::Nonterminal(root));
    let terminals = grammar.terminals.clone();
    let mut table = Table::build(grammar);
    for state in &mut table.states {
        state.first.nullable = true; // Disable FIRST pruning.
        // Independent predicate evaluation, without interval merging, for the
        // exhaustive test alphabet. Terminal sharing is disabled above.
        state.dispatch = ['a', 'b', 'z', '🦀']
            .into_iter()
            .map(|ch| ShiftRange {
                lo: ch as u32,
                hi: ch as u32,
                targets: state
                    .shifts
                    .iter()
                    .filter(|(id, _)| terminals[**id].matches(ch))
                    .map(|(_, &target)| target)
                    .collect(),
            })
            .collect();
    }
    let baseline = Cluster::<Root> {
        table,
        _shard: PhantomData,
    };
    let mut words = vec![String::new()];
    for _ in 0..5 {
        let mut next = Vec::new();
        for word in words {
            let collect = |cluster: &Cluster<Root>| {
                cluster
                    .parse(&word)
                    .map(|parsed| parsed.result().unwrap())
                    .ok()
            };
            assert_eq!(collect(&optimized), collect(&baseline), "{word:?}");
            for ch in ['a', 'b', 'z', '🦀'] {
                next.push(format!("{word}{ch}"));
            }
        }
        words = next;
    }
}
