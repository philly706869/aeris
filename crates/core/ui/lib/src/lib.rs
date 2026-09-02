pub mod internal;

use std::marker::PhantomData;

use crate::internal::{ShardDataType, StaticShard};

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
    pub const fn build() -> Self {
        let data = <S::DATA as ShardDataType>::DATA;
        Self {
            _shard: PhantomData,
        }
    }

    pub fn parse<'i>(&self, input: &'i str) -> S {
        todo!()
    }
}
