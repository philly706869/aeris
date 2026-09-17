#![allow(dead_code)]

use xst::{Cluster, ExtractError, shard};

#[shard]
type Rune = x! { [| "한" | "🦀"] };

#[shard]
struct Generic<T> {
    item: T,
}

#[shard]
struct Record {
    bracket: x! { "[" },
    values: xvec![Generic<xbox![Rune]>, 1..3],
    option: xopt![x! { "!" }],
    tuple: (x! { ":" }, x! { "끝" }),
    singleton: (x! { "." },),
    empty: (),
    bracket: x! { "]" },
}

#[test]
fn maps_struct_wrappers_generics_and_duplicate_fields() {
    let input = String::from("[한🦀!:끝.]");
    let output = {
        let cluster = Cluster::<Record>::build();
        let parsed = cluster.parse(&input).unwrap();
        let mut results = parsed.results().unwrap();
        let output = results.next().unwrap().unwrap();
        assert!(results.next().is_none());
        output
    };
    assert_eq!(output.bracket, ("[", "]"));
    assert_eq!(*output.values[0].item, "한");
    assert_eq!(*output.values[1].item, "🦀");
    assert_eq!(output.values[0].item.as_ptr(), input[1..].as_ptr());
    assert_eq!(output.values[1].item.as_ptr(), input[4..].as_ptr());
    assert_eq!(output.option, Some("!"));
    assert_eq!(output.tuple, (":", "끝"));
    assert_eq!(output.singleton, (".",));
}

#[shard]
struct EmptyOption {
    value: xopt![x! { "" }],
}

#[test]
fn distinguishes_none_from_some_empty() {
    let cluster = Cluster::<EmptyOption>::build();
    let parsed = cluster.parse("").unwrap();
    let values: Vec<_> = parsed
        .results()
        .unwrap()
        .map(|r| r.unwrap().value)
        .collect();
    assert_eq!(values.len(), 2);
    assert!(values.contains(&None));
    assert!(values.contains(&Some("")));
}

#[shard]
type Same = x! { [| "a" | {'a'}] };
#[shard]
struct Pair {
    left: Same,
    right: Same,
}

#[test]
fn keeps_duplicate_values_and_cartesian_derivations() {
    let cluster = Cluster::<Pair>::build();
    let parsed = cluster.parse("aa").unwrap();
    let values: Vec<_> = parsed.results().unwrap().map(Result::unwrap).collect();
    assert_eq!(values.len(), 4);
    assert!(values.iter().all(|v| v.left == "a" && v.right == "a"));
}

#[shard]
type Ambiguous = x! { [| Ambiguous Ambiguous | "a"] };

#[test]
fn enumerates_catalan_derivations_and_is_lazy() {
    let cluster = Cluster::<Ambiguous>::build();
    for (input, count) in [("a", 1), ("aa", 1), ("aaa", 2), ("aaaa", 5), ("aaaaa", 14)] {
        let parsed = cluster.parse(input).unwrap();
        let mut actual = 0;
        for value in parsed.results().unwrap() {
            assert_eq!(value.unwrap(), input);
            actual += 1;
        }
        assert_eq!(actual, count, "{input}");
    }
    // There are 1,767,263,190 derivations; next() must not enumerate them first.
    let input = "a".repeat(20);
    let parsed = cluster.parse(&input).unwrap();
    assert_eq!(parsed.results().unwrap().next().unwrap().unwrap(), input);
}

#[shard]
type Cycle = x! { [| Cycle | "a"] };
#[shard]
type EmptyCycle = x! { ""* };
#[shard]
type FailedCycle = x! { [| Cycle "x" | "a"] };

#[test]
fn only_productive_accepted_cycles_block_extraction() {
    let cluster = Cluster::<Cycle>::build();
    assert!(cluster.recognizes("a"));
    assert!(matches!(
        cluster.parse("a").unwrap().results(),
        Err(ExtractError::InfiniteDerivations)
    ));
    let cluster = Cluster::<EmptyCycle>::build();
    assert!(matches!(
        cluster.parse("").unwrap().results(),
        Err(ExtractError::InfiniteDerivations)
    ));
    let cluster = Cluster::<FailedCycle>::build();
    let parsed = cluster.parse("a").unwrap();
    assert_eq!(parsed.results().unwrap().next().unwrap().unwrap(), "a");
}

#[shard]
type A = x! { "a" };
#[shard]
type AlsoA = x! { {'a'} };
#[shard]
enum Choice {
    First(A),
    Second(AlsoA),
}

#[test]
fn variants_survive_terminal_sharing_in_declaration_order() {
    let cluster = Cluster::<Choice>::build();
    let parsed = cluster.parse("a").unwrap();
    let mut results = parsed.results().unwrap();
    assert!(matches!(
        results.next().unwrap().unwrap(),
        Choice::First("a")
    ));
    assert!(matches!(
        results.next().unwrap().unwrap(),
        Choice::Second("a")
    ));
    assert!(results.next().is_none());
}

#[shard]
struct NestedClosure {
    value: Generic<xopt![Generic<xvec![Rune, 1..3]>]>,
}

#[test]
fn maps_nested_generic_closures() {
    let cluster = Cluster::<NestedClosure>::build();
    let parsed = cluster.parse("한🦀").unwrap();
    let value = parsed.results().unwrap().next().unwrap().unwrap();
    assert_eq!(value.value.item.unwrap().item, vec!["한", "🦀"]);
}

#[shard]
type Bytes = x! { "한🦀!" };
#[test]
fn parse_errors_report_byte_offsets() {
    let cluster = Cluster::<Bytes>::build();
    assert_eq!(cluster.parse("한🦀?").err().unwrap().offset, 7);
    assert_eq!(cluster.parse("한").err().unwrap().offset, 3);
    assert_eq!(cluster.parse("한🦀!x").err().unwrap().offset, 8);
}

mod json {
    include!("fixtures/json.rs");
    #[test]
    fn maps_nested_json() {
        let cluster = xst::Cluster::<JSON>::build();
        let parsed = cluster.parse(r#"{"한🦀":[true,null,12.5]}"#).unwrap();
        let output = parsed.results().unwrap().next().unwrap().unwrap();
        let JSONValue::Object(object) = output.value else {
            panic!("object expected")
        };
        let (entry, tail) = object.content.inner.unwrap();
        assert!(tail.is_empty());
        assert_eq!(entry.inner.name.content, "한🦀");
        let JSONValue::Array(array) = entry.inner.value else {
            panic!("array expected")
        };
        let (first, tail) = array.entries.inner.unwrap();
        assert!(matches!(
            first.inner,
            JSONValue::Boolean(JSONBoolean::True(_))
        ));
        assert!(matches!(tail[0].1.inner, JSONValue::Null(_)));
        let JSONValue::Number(number) = &tail[1].1.inner else {
            panic!("number expected")
        };
        assert_eq!(number.integer, "12");
        assert_eq!(number.fraction.as_ref().unwrap().digits, "5");
    }
}

#[test]
fn mapping_occurs_only_when_next_is_requested() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use xst::internal::{MappingNode, Shard, ShardCore, ShardData, StaticShard};
    static CALLS: AtomicUsize = AtomicUsize::new(0);
    struct Counted;
    impl Shard for Counted {
        type Core = Self;
    }
    impl StaticShard for Counted {}
    impl ShardCore for Counted {
        type Output<'i> = &'i str;
        const DATA: &'static ShardData =
            &ShardData::alternative(&[&ShardData::literal("a"), &ShardData::literal("a")]);
        fn map<'i>(node: MappingNode<'_, 'i>) -> Result<&'i str, ExtractError> {
            CALLS.fetch_add(1, Ordering::SeqCst);
            node.slice()
        }
    }
    let cluster = Cluster::<Counted>::build();
    let parsed = cluster.parse("a").unwrap();
    let mut results = parsed.results().unwrap();
    assert_eq!(CALLS.load(Ordering::SeqCst), 0);
    assert_eq!(results.next().unwrap().unwrap(), "a");
    assert_eq!(CALLS.load(Ordering::SeqCst), 1);
    drop(results);
    assert_eq!(CALLS.load(Ordering::SeqCst), 1);
}

#[test]
fn mismatched_mapping_returns_an_error() {
    use xst::internal::{MappingNode, Shard, ShardCore, ShardData, StaticShard};
    struct Wrong;
    impl Shard for Wrong {
        type Core = Self;
    }
    impl StaticShard for Wrong {}
    impl ShardCore for Wrong {
        type Output<'i> = &'i str;
        const DATA: &'static ShardData = &ShardData::literal("a");
        fn map<'i>(node: MappingNode<'_, 'i>) -> Result<&'i str, ExtractError> {
            node.reference::<A<'static>>()
        }
    }
    let cluster = Cluster::<Wrong>::build();
    let parsed = cluster.parse("a").unwrap();
    assert_eq!(
        parsed.results().unwrap().next().unwrap(),
        Err(ExtractError::InvalidMapping)
    );
}

#[shard]
struct EmptyRecord {}
#[shard]
struct FiniteEmpty {
    items: xvec![x! { "" }, 0..2],
}
#[shard]
struct Unbounded {
    items: xvec![Rune, 1..],
}

#[test]
fn restores_empty_bounded_and_unbounded_repetition() {
    let cluster = Cluster::<EmptyRecord>::build();
    assert!(
        cluster
            .parse("")
            .unwrap()
            .results()
            .unwrap()
            .next()
            .unwrap()
            .is_ok()
    );
    let cluster = Cluster::<FiniteEmpty>::build();
    let parsed = cluster.parse("").unwrap();
    let mut lengths: Vec<_> = parsed
        .results()
        .unwrap()
        .map(|r| r.unwrap().items.len())
        .collect();
    lengths.sort();
    assert_eq!(lengths, vec![0, 1, 2]);
    let cluster = Cluster::<Unbounded>::build();
    let parsed = cluster.parse("한🦀한").unwrap();
    let value = parsed.results().unwrap().next().unwrap().unwrap();
    assert_eq!(value.items, vec!["한", "🦀", "한"]);
}
