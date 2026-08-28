use std::any::TypeId;

pub trait Shard
where
    Self::TypeId: 'static,
{
    type TypeId;
    const DATA: &'static ShardData;
}

pub enum ShardData {}

pub const fn build<S>()
where
    S: Shard,
{
}
