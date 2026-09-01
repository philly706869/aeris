use std::{any::TypeId, marker::PhantomData, ops::RangeInclusive};

///
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
    _shard: PhantomData<S>,
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
}

pub trait StaticShard: Shard {
    const DATA: &'static ShardData;
}

pub trait DynamicShard: Shard {}

pub enum ShardData {
    Literal(&'static str),
    Set(bool, &'static [RangeInclusive<char>]),
    Sequence(&'static [&'static ShardData]),
    Alternative(&'static [&'static ShardData]),
    Vector(&'static ShardData, usize, usize),
    Option(&'static ShardData),
    Extern(TypeId, fn() -> &'static ShardData),
}
