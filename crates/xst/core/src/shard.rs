use core::{any::TypeId, fmt::Debug, ops::RangeInclusive};

/// Internal API for generated code.
/// This module is not intended for direct use.
#[doc(hidden)]
pub mod internal {
    pub use core::fmt::Debug;
    pub use core::marker::PhantomData;
    pub use core::ops::RangeInclusive;
    pub use core::option::Option;
    pub use core::primitive::char;
    pub use core::primitive::str;

    extern crate alloc;
    pub use alloc::boxed::Box;
    pub use alloc::vec::Vec;

    pub use super::Shard;
    pub use super::ShardCore;
    pub use super::ShardData;
    pub use super::ShardField;
    pub use super::StaticShard;
}

pub trait StaticShard: Shard {}

pub trait Shard: 'static {
    type Core: ShardCore;
}

pub trait ShardCore: 'static {
    type Output<'i>;
    const DATA: &'static ShardData;
}

pub type ShardField<'i, T> = <<T as Shard>::Core as ShardCore>::Output<'i>;

#[derive(Debug)]
pub struct ShardData {
    kind: ShardDataKind,
}

impl ShardData {
    const fn new(kind: ShardDataKind) -> Self {
        Self { kind }
    }

    pub const fn literal(text: &'static str) -> Self {
        Self::new(ShardDataKind::Literal(LiteralData::new(text)))
    }

    pub const fn set(negated: bool, range: &'static [RangeInclusive<char>]) -> Self {
        Self::new(ShardDataKind::Set(SetData::new(negated, range)))
    }

    pub const fn option(item: &'static ShardData) -> Self {
        Self::new(ShardDataKind::Option(OptionData::new(item)))
    }

    pub const fn vec(item: &'static ShardData, min: usize, max: usize) -> Self {
        Self::new(ShardDataKind::Vec(VecData::new(item, min, max)))
    }

    pub const fn sequence(items: &'static [&'static ShardData]) -> Self {
        Self::new(ShardDataKind::Sequence(SequenceData::new(items)))
    }

    pub const fn alternative(items: &'static [&'static ShardData]) -> Self {
        Self::new(ShardDataKind::Alternative(AlternativeData::new(items)))
    }

    pub const fn reference<T>() -> Self
    where
        T: Shard,
    {
        Self::new(ShardDataKind::Reference(ReferenceData::new::<T>()))
    }
}

#[derive(Debug)]
pub enum ShardDataKind {
    Literal(LiteralData),
    Set(SetData),
    Option(OptionData),
    Vec(VecData),
    Sequence(SequenceData),
    Alternative(AlternativeData),
    Reference(ReferenceData),
}

#[derive(Debug)]
pub struct LiteralData {
    text: &'static str,
}

impl LiteralData {
    pub const fn new(text: &'static str) -> Self {
        Self { text }
    }
}

#[derive(Debug)]
pub struct SetData {
    negated: bool,
    range: &'static [RangeInclusive<char>],
}

impl SetData {
    pub const fn new(negated: bool, range: &'static [RangeInclusive<char>]) -> Self {
        Self { negated, range }
    }
}

#[derive(Debug)]
pub struct OptionData {
    item: &'static ShardData,
}

impl OptionData {
    pub const fn new(item: &'static ShardData) -> Self {
        Self { item }
    }
}

#[derive(Debug)]
pub struct VecData {
    item: &'static ShardData,
    min: usize,
    max: usize,
}

impl VecData {
    pub const fn new(item: &'static ShardData, min: usize, max: usize) -> Self {
        Self { item, min, max }
    }
}

#[derive(Debug)]
pub struct SequenceData {
    items: &'static [&'static ShardData],
}

impl SequenceData {
    pub const fn new(items: &'static [&'static ShardData]) -> Self {
        Self { items }
    }
}

#[derive(Debug)]
pub struct AlternativeData {
    items: &'static [&'static ShardData],
}

impl AlternativeData {
    pub const fn new(items: &'static [&'static ShardData]) -> Self {
        Self { items }
    }
}

#[derive(Debug)]
pub struct ReferenceData {
    id: TypeId,
    reference: fn() -> &'static ShardData,
}

impl ReferenceData {
    pub const fn new<T>() -> Self
    where
        T: Shard,
    {
        Self {
            id: TypeId::of::<T::Core>(),
            reference: || T::Core::DATA,
        }
    }
}

fn normalize_set(set: &SetData) -> Vec<RangeInclusive<char>> {
    let mut ranges = Vec::from(set.range);
    ranges.sort_unstable_by_key(|r| (*r.start(), *r.end()));
    let mut normalized: Vec<RangeInclusive<char>> = Vec::with_capacity(ranges.len());
    for range in ranges {
        let start = *range.start();
        let end = *range.end();
        match normalized.last_mut() {
            Some(last) => {
                let last_end = *last.end();
                if start as u32 <= last_end as u32 + 1 && end > last_end {
                    *last = *last.start()..=end;
                } else {
                    normalized.push(range);
                }
            }
            None => normalized.push(range),
        }
    }
    normalized
}
