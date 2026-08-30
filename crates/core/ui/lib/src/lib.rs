use std::{any::TypeId, ops::RangeInclusive};

pub trait Shard {
    const TUID: TypeId;
    const DATA: &'static ShardData;
}

mod sealed {
    pub trait Sealed {}
}

use sealed::Sealed;

pub trait ShardEntry
where
    Self: Sealed,
{
}

impl<T> Sealed for T where T: Shard {}
impl<T> ShardEntry for T where T: Shard {}

impl<'t> Sealed for &'t str {}
impl<'t> ShardEntry for &'t str {}

pub enum ShardData {
    Literal(&'static str),
    Set(bool, Vec<RangeInclusive<char>>),
    Sequence(Vec<ShardData>),
    Alternative(Vec<ShardData>),
}

pub const fn build<S>()
where
    S: Shard,
{
}
