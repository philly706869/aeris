#![no_implicit_prelude]
#![allow(dead_code, non_camel_case_types)]

extern crate std;
extern crate xst;

// These names must never be resolved by generated library references.
struct Option;
struct Box;
struct Vec;
struct PhantomData;
struct Shard;
struct ShardCore;
struct str;
mod fmt {}

#[::xst::shard]
pub struct Generic<T> {
    item: T,
}

#[::xst::shard]
pub struct Root {
    __xst_marker_0: x! { "a" },
    item: Generic<xoptz![xbox![xvecz![x! { "b" }, ..2]]]>,
}

#[test]
fn expands_without_prelude_or_unqualified_library_names() {
    let _ = ::xst::Cluster::<Root<'static>>::build();
    let _: ::xst::internal::ShardField<'_, ::xst::internal::ShardClosure<Root<'static>, 0>> =
        ::xst::internal::Option::None;
}

#[::xst::shard]
pub struct __xst_shard_core_0 {
    text: x! { "core" },
}

#[::xst::shard]
pub struct __xst_shard_core_1 {
    text: x! { "core1" },
}

#[::xst::shard]
pub struct __xst_shard_closure_0 {
    text: x! { "closure" },
}

#[::xst::shard]
pub struct Collisions {
    generated: Generic<x! { "generated" }>,
    nested: Generic<xopt![Generic<x! { "nested" }>]>,
    core: __xst_shard_core_0,
    another_core: __xst_shard_core_1,
    closure: r#__xst_shard_closure_0,
    __xst_marker_0: x! { "marker0" },
    r#__xst_marker_1: x! { "marker1" },
}

#[::xst::shard]
pub struct GenericCollisions<__xst_shard_core_0, __xst_shard_closure_0> {
    first: __xst_shard_core_0,
    second: __xst_shard_closure_0,
    nested: Generic<xbox![__xst_shard_core_0]>,
}

#[::xst::shard]
type PrimitiveCollision = x! { Generic<"x"> r#__xst_shard_closure_0 __xst_shard_core_0 };

#[test]
fn generated_names_do_not_shadow_input_names() {
    let _ = ::xst::Cluster::<Collisions<'static>>::build();
    let _: ::xst::internal::ShardField<'_, ::xst::internal::ShardClosure<Collisions<'static>, 0>> =
        "generated";
    let _: ::xst::internal::ShardField<'_, ::xst::internal::ShardClosure<Collisions<'static>, 2>> =
        "nested";
    let _: ::xst::internal::ShardField<
        '_,
        ::xst::internal::ShardClosure<PrimitiveCollision<'static>, 0>,
    > = "x";
    type Captured =
        GenericCollisions<'static, __xst_shard_core_0<'static>, __xst_shard_closure_0<'static>>;
    let _: ::xst::internal::ShardField<'_, ::xst::internal::ShardClosure<Captured, 0>> =
        ::xst::internal::Box::new(__xst_shard_core_0 {
            __xst_marker_0: ::std::default::Default::default(),
            text: "core",
        });
}
