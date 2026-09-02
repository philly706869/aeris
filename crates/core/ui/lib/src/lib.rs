pub mod internal;

use std::{any::TypeId, collections::hash_map::Entry, marker::PhantomData};

use rustc_hash::FxHashMap;

use crate::internal::{ShardData, ShardDataType, StaticShard};

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
        let data = <S::DATA as ShardDataType>::DATA;

        let mut stack = vec![data];
        let mut table: FxHashMap<TypeId, &ShardData> = FxHashMap::default();

        table.insert(TypeId::of::<S::DATA>(), data);

        while let Some(data) = stack.pop() {
            match data {
                ShardData::Literal(_) => {}
                ShardData::Set(_) => {}
                ShardData::Option(data) => stack.push(data),
                ShardData::Vector(data) => stack.push(data.item),
                ShardData::Sequence(data) => stack.extend(data.iter()),
                ShardData::Alternative(data) => stack.extend(data.iter()),
                ShardData::Extern(type_id, data) => {
                    if let Entry::Vacant(entry) = table.entry(type_id.to_owned()) {
                        let data = data();
                        entry.insert(data);
                        stack.push(data);
                    }
                }
            }
        }

        dbg!(table);

        Self {
            _shard: PhantomData,
        }
    }

    pub fn call(&self) {
        println!();
    }

    pub fn parse<'i>(&self, input: &'i str) -> S {
        todo!()
    }
}
