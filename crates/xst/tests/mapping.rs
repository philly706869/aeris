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
        parsed.result().unwrap()
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
    assert_eq!(cluster.parse("").unwrap().result().unwrap().value, Some(""));
}

#[shard]
type Same = x! { [| "a" | {'a'}] };
#[shard]
struct Pair {
    left: Same,
    right: Same,
}

#[test]
fn selects_one_value_from_ambiguous_fields() {
    let cluster = Cluster::<Pair>::build();
    let parsed = cluster.parse("aa").unwrap();
    let value = parsed.result().unwrap();
    assert_eq!((value.left, value.right), ("a", "a"));
}

#[shard]
type Ambiguous = x! { [| Ambiguous Ambiguous | "a"] };

#[test]
fn selects_without_enumerating_catalan_derivations() {
    let cluster = Cluster::<Ambiguous>::build();
    for input in ["a", "aa", "aaa", "aaaa", "aaaaa"] {
        assert_eq!(cluster.parse(input).unwrap().result().unwrap(), input);
    }
    // Select from 1,767,263,190 derivations without enumerating all of them.
    let input = "a".repeat(20);
    assert_eq!(cluster.parse(&input).unwrap().result().unwrap(), input);
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
        cluster.parse("a").unwrap().result(),
        Err(ExtractError::InfiniteDerivations)
    ));
    let cluster = Cluster::<EmptyCycle>::build();
    assert!(matches!(
        cluster.parse("").unwrap().result(),
        Err(ExtractError::InfiniteDerivations)
    ));
    let cluster = Cluster::<FailedCycle>::build();
    let parsed = cluster.parse("a").unwrap();
    assert_eq!(parsed.result().unwrap(), "a");
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
    assert!(matches!(parsed.result().unwrap(), Choice::First("a")));
}

#[shard]
struct NestedClosure {
    value: Generic<xopt![Generic<xvec![Rune, 1..3]>]>,
}

#[test]
fn maps_nested_generic_closures() {
    let cluster = Cluster::<NestedClosure>::build();
    let parsed = cluster.parse("한🦀").unwrap();
    let value = parsed.result().unwrap();
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
        let output = parsed.result().unwrap();
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
fn mapping_occurs_only_for_the_requested_result() {
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
    assert_eq!(CALLS.load(Ordering::SeqCst), 0);
    assert_eq!(parsed.result().unwrap(), "a");
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
    assert_eq!(parsed.result(), Err(ExtractError::InvalidMapping));
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
    assert!(cluster.parse("").unwrap().result().is_ok());
    let cluster = Cluster::<FiniteEmpty>::build();
    assert_eq!(cluster.parse("").unwrap().result().unwrap().items.len(), 2);
    let cluster = Cluster::<Unbounded>::build();
    let value = cluster.parse("한🦀한").unwrap().result().unwrap();
    assert_eq!(value.items, vec!["한", "🦀", "한"]);
}

#[shard]
struct GreedySplit {
    first: xvec![x! { "a" }, ..],
    rest: x! { "a"* },
}
#[shard]
struct LazySplit {
    first: xvecz![x! { "a" }, ..],
    rest: x! { "a"* },
}
#[shard]
struct GreedyOption {
    first: xopt![x! { "a" }],
    rest: x! { "a"* },
}
#[shard]
struct LazyOption {
    first: xoptz![x! { "a" }],
    rest: x! { "a"* },
}
#[shard]
struct LazyEmptyOption {
    first: xoptz![x! { "" }],
}
#[shard]
struct LazyEmptyVec {
    first: xvecz![x! { "" }, 0..2],
}

#[test]
fn greedy_and_lazy_prefer_opposite_branches() {
    let value = Cluster::<GreedySplit>::build()
        .parse("aaa")
        .unwrap()
        .result()
        .unwrap();
    assert_eq!(value.first, vec!["a", "a", "a"]);
    assert_eq!(value.rest, "");
    let value = Cluster::<LazySplit>::build()
        .parse("aaa")
        .unwrap()
        .result()
        .unwrap();
    assert!(value.first.is_empty());
    assert_eq!(value.rest, "aaa");
    let value = Cluster::<GreedyOption>::build()
        .parse("a")
        .unwrap()
        .result()
        .unwrap();
    assert_eq!((value.first, value.rest), (Some("a"), ""));
    let value = Cluster::<LazyOption>::build()
        .parse("a")
        .unwrap()
        .result()
        .unwrap();
    assert_eq!((value.first, value.rest), (None, "a"));
    assert_eq!(
        Cluster::<LazyEmptyOption>::build()
            .parse("")
            .unwrap()
            .result()
            .unwrap()
            .first,
        None
    );
    assert!(
        Cluster::<LazyEmptyVec>::build()
            .parse("")
            .unwrap()
            .result()
            .unwrap()
            .first
            .is_empty()
    );
}

#[shard]
struct BacktrackGreedy {
    first: xvec![x! { "a" }, ..],
    required: x! { "a" },
}
#[shard]
struct BacktrackLazy {
    first: xvecz![x! { "a" }, ..],
    required: x! { "a" },
}
#[shard]
type LongFirst = x! { [| "aa" | "a"] };
#[shard]
type ShortFirst = x! { [| "a" | "aa"] };
#[shard]
struct EarlierLong {
    first: LongFirst,
    rest: x! { "a"* },
}
#[shard]
struct EarlierShort {
    first: ShortFirst,
    rest: x! { "a"* },
}
#[shard]
struct Fallback {
    first: LongFirst,
    required: x! { "a" },
}

#[test]
fn priority_uses_grammar_order_and_requires_complete_success() {
    let value = Cluster::<EarlierLong>::build()
        .parse("aa")
        .unwrap()
        .result()
        .unwrap();
    assert_eq!((value.first, value.rest), ("aa", ""));
    let value = Cluster::<EarlierShort>::build()
        .parse("aa")
        .unwrap()
        .result()
        .unwrap();
    assert_eq!((value.first, value.rest), ("a", "a"));
    let value = Cluster::<Fallback>::build()
        .parse("aa")
        .unwrap()
        .result()
        .unwrap();
    assert_eq!(value.first, "a");
    let value = Cluster::<BacktrackGreedy>::build()
        .parse("aaa")
        .unwrap()
        .result()
        .unwrap();
    assert_eq!(value.first.len(), 2);
    let value = Cluster::<BacktrackLazy>::build()
        .parse("aaa")
        .unwrap()
        .result()
        .unwrap();
    assert_eq!(value.first.len(), 2);
}

#[shard]
struct BoundedGreedy {
    first: xvec![x! { "a" }, 1..2],
    rest: x! { "a"* },
}
#[shard]
struct BoundedLazy {
    first: xvecz![x! { "a" }, 1..2],
    rest: x! { "a"* },
}
#[shard]
struct PrimitiveLazy {
    first: x! { "a"*? },
    rest: x! { "a"* },
}
#[shard]
struct PrimitiveGreedy {
    first: x! { "a"* },
    rest: x! { "a"* },
}

#[test]
fn bounded_and_primitive_repetitions_use_branch_priority() {
    assert_eq!(
        Cluster::<BoundedGreedy>::build()
            .parse("aaa")
            .unwrap()
            .result()
            .unwrap()
            .first
            .len(),
        2
    );
    assert_eq!(
        Cluster::<BoundedLazy>::build()
            .parse("aaa")
            .unwrap()
            .result()
            .unwrap()
            .first
            .len(),
        1
    );
    assert_eq!(
        Cluster::<PrimitiveLazy>::build()
            .parse("aaa")
            .unwrap()
            .result()
            .unwrap()
            .first,
        ""
    );
    assert_eq!(
        Cluster::<PrimitiveGreedy>::build()
            .parse("aaa")
            .unwrap()
            .result()
            .unwrap()
            .first,
        "aaa"
    );
}
