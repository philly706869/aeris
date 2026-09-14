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
