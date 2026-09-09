#[doc(hidden)]
pub mod internal {
    pub use core::primitive::char;
    pub use core::primitive::str;

    pub use std::marker::PhantomData;
    pub use std::ops::RangeInclusive;

    pub use crate::shard::Shard;
    pub use crate::shard::ShardLiteral;
    pub use crate::shard::ShardParam;
    pub use crate::shard::ShardSet;
    pub use crate::shard::StaticShard;

    pub use crate::shard::AlternativeType as Alternative;
    pub use crate::shard::ExternType as Extern;
    pub use crate::shard::LiteralType as Literal;
    pub use crate::shard::OptionType as Option;
    pub use crate::shard::SequenceType as Sequence;
    pub use crate::shard::SetType as Set;
    pub use crate::shard::VecType as Vec;
}

mod shard;

use std::{any::TypeId, collections::hash_map::Entry, marker::PhantomData};

use rustc_hash::FxHashMap;

use crate::shard::{ShardData, ShardDataType, StaticShard};

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
        let data = <shard::ExternType<S>>::data();
        let mut stack: Vec<&ShardData> = vec![&data.into()];
        let mut table: FxHashMap<TypeId, &ShardData> = FxHashMap::default();

        while let Some(data) = stack.pop() {
            match data {
                ShardData::Literal(_) => {}
                ShardData::Set(_) => {}
                ShardData::Option(data) => {
                    stack.push(&*data.item);
                }
                ShardData::Vec(data) => {
                    stack.push(&*data.item);
                }
                ShardData::Sequence(data) => {
                    stack.extend(data.items.iter());
                }
                ShardData::Alternative(data) => {
                    stack.extend(data.items.iter());
                }
                ShardData::Extern(data) => {
                    if let Entry::Vacant(entry) = table.entry(data.id) {
                        let data = (data.reference)();
                        entry.insert(&data);
                        stack.push(&data);
                    }
                }
            }
        }

        Self {
            _shard: PhantomData,
        }
    }

    pub fn parse<'i>(&self, input: &'i str) -> S {
        todo!()
    }
}
