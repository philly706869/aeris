use std::{any::TypeId, marker::PhantomData, ops::RangeInclusive};

#[macro_export]
macro_rules! x {
    () => {};
}

#[macro_export]
macro_rules! cluster {
    () => {};
}

pub struct Cluster<S>
where
    S: StaticShard,
{
    _shard: PhantomData<fn() -> S>,
}

impl<S> Cluster<S>
where
    S: StaticShard,
{
    pub fn build() -> Self {
        Self {
            _shard: PhantomData,
        }
    }

    pub fn parse<'i>(&self, input: &'i str) -> S {
        todo!()
    }
}

pub trait Shard {
    type TUID: 'static;
    const DATA: &'static ShardData;
}

pub trait StaticShard: Shard {}

pub enum ShardData {
    Literal(&'static str),
    Set(bool, &'static [RangeInclusive<char>]),
    Sequence(&'static [&'static ShardData]),
    Alternative(&'static [&'static ShardData]),
    Option(&'static ShardData),
    Vector(&'static ShardData, usize, usize),
    Extern(TypeId, fn() -> &'static ShardData),
}
