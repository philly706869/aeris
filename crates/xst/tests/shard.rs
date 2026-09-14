#![allow(dead_code)]

mod json {
    include!("fixtures/json.rs");

    #[test]
    fn recursive_mapping_examples_compile() {
        let _ = xst::Cluster::<JSON<'static>>::build();
        let _: xst::internal::ShardField<'_, Digit<'static>> = "3";
        let _: xst::internal::ShardField<'_, xst::internal::ShardClosure<JSONObject<'static>, 0>> =
            ",";
        let _: xst::internal::ShardField<'_, xst::internal::ShardClosure<JSONArray<'static>, 0>> =
            ",";
    }
}

use xst::{
    internal::{Shard, ShardClosure, ShardCore, ShardField},
    shard,
};

#[shard]
type Letter = x! { {'a'..'z'} };

#[shard]
pub struct Identity<T> {
    value: T,
}

#[shard]
enum Choice<T> {
    Value(T),
    Nested(Identity<xbox![T]>),
}

#[shard]
struct Capture<T> {
    first: Identity<xbox![T]>,
    nested: Identity<xopt![Identity<xvecz![T, 1..3]>]>,
    pair: Identity<(T, x! { "," })>,
}

#[shard]
struct Duplicate {
    token: x! { "[" },
    middle: xopt![Letter],
    token: x! { "]" },
}

#[shard]
struct Runes {
    boxed: xbox![Letter],
    opt: xopt![Letter],
    optz: xoptz![Letter],
    exact: xvec![Letter, 3],
    range: xvec![Letter, 2..4],
    vecz: xvecz![Letter, ..2],
    single: (Letter,),
    empty: (),
}

#[shard]
type Primitive<T> = x! {
    Identity<"a"> Identity<T> Identity<T+?>
    "b"*? "c"+? "d"^[2..4?] "e"^[3] ("f" "g")
    [| "h" | ] {! '0'..'9'}
};

#[shard]
struct Unused<T> {}

#[shard]
type UnusedType<T> = x! { "x" };

#[test]
fn nested_closures_capture_generic_shards() {
    type Root = Capture<'static, Letter<'static>>;
    let _: ShardField<'_, ShardClosure<Root, 0>> = Box::new("a");
    let _: ShardField<'_, ShardClosure<Root, 2>> = vec!["a", "b"];
    let _: ShardField<'_, ShardClosure<Root, 1>> = Some(Identity {
        __xst_marker_0: Default::default(),
        value: vec!["a"],
    });
    let _: ShardField<'_, ShardClosure<Root, 3>> = ("a", ",");
    let _ = <Root as Shard>::Core::DATA;
    let _ = <Primitive<'static, Letter<'static>> as Shard>::Core::DATA;
    let choice = Choice::<'_, Letter<'static>>::Value("a");
    assert_eq!(format!("{choice:?}"), "Value(\"a\")");
    let nested = Choice::<'_, Letter<'static>>::Nested(Identity {
        __xst_marker_0: Default::default(),
        value: Box::new("b"),
    });
    assert_eq!(format!("{nested:?}"), "Nested(Identity { value: \"b\" })");
}

#[test]
fn instantiated_generic_shards_are_not_static() {
    trait AmbiguousIfStatic<A> {
        fn check() {}
    }
    impl<T> AmbiguousIfStatic<()> for T {}
    impl<T: xst::internal::StaticShard> AmbiguousIfStatic<u8> for T {}

    // These calls become ambiguous if a generic shard implements StaticShard.
    let _ = <Identity<'static, Letter<'static>> as AmbiguousIfStatic<_>>::check;
    let _ = <Choice<'static, Letter<'static>> as AmbiguousIfStatic<_>>::check;
    let _ = <Primitive<'static, Letter<'static>> as AmbiguousIfStatic<_>>::check;
}

#[test]
fn duplicate_fields_preserve_debug_order() {
    let value = Duplicate {
        __xst_marker_0: Default::default(),
        token: ("[", "]"),
        middle: Some("a"),
    };
    assert_eq!(
        format!("{value:?}"),
        "Duplicate { token: \"[\", middle: Some(\"a\"), token: \"]\" }"
    );
}

#[test]
fn rune_output_types_and_inclusive_bounds() {
    let value = Runes {
        __xst_marker_0: Default::default(),
        boxed: Box::new("a"),
        opt: Some("a"),
        optz: None,
        exact: vec!["a"; 3],
        range: vec!["a"; 4],
        vecz: vec![],
        single: ("a",),
        empty: (),
    };
    assert_eq!(value.range.len(), 4);
    let data = format!("{:?}", <Runes<'static> as Shard>::Core::DATA);
    assert!(data.contains("min: 3, max: Some(3)"));
    assert!(data.contains("min: 2, max: Some(4)"));
    assert!(data.contains("min: 0, max: Some(2)"));
}

#[shard]
struct Lazy {
    option: xoptz![x! { "option" }],
    vec: xvecz![x! { "vec" }, 2..4],
    star: x! { "star"*? },
    plus: x! { "plus"+? },
    range: x! { "range"^[1..3?] },
    unbounded: x! { "unbounded"^[..?] },
    nested: Identity<xoptz![xvec![x! { "nested" }, 1..2]]>,
}

#[test]
fn lazy_flags_reach_grammar_data_and_closures() {
    let data = format!("{:?}", <Lazy<'static> as Shard>::Core::DATA);
    assert_eq!(data.matches("lazy: true").count(), 6);
    assert_eq!(data.matches("lazy: false").count(), 0);
    for bounds in [
        "min: 2, max: Some(4), lazy: true",
        "min: 0, max: None, lazy: true",
        "min: 1, max: None, lazy: true",
        "min: 1, max: Some(3), lazy: true",
    ] {
        assert!(data.contains(bounds), "{data}");
    }

    type Nested = ShardClosure<Lazy<'static>, 0>;
    let nested = format!("{:?}", <Nested as Shard>::Core::DATA);
    assert!(nested.contains("min: 1, max: Some(2), lazy: false"));
    assert!(nested.ends_with(", lazy: true }) }"));
    let _: ShardField<'_, Nested> = Some(vec!["nested"]);

    let greedy = format!("{:?}", <Runes<'static> as Shard>::Core::DATA);
    assert_eq!(greedy.matches("lazy: false").count(), 3);
    assert_eq!(greedy.matches("lazy: true").count(), 2);
}
