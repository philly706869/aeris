use core::marker::PhantomData;

use crate::shard::{self, StaticShard};

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
        let reference = shard::ReferenceData::new::<S>();

        Self {
            _shard: PhantomData,
        }
    }

    pub fn parse(&self, input: &str) {
        todo!()
    }
}
