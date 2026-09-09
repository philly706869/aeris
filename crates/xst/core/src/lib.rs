/// Internal API for generated code.
/// This module is not intended for direct use.
#[doc(hidden)]
pub mod internal {
    pub use core::marker::PhantomData;
    pub use core::ops::RangeInclusive;
    pub use core::primitive::char;
    pub use core::primitive::str;

    pub use crate::shard::Shard;
    pub use crate::shard::ShardLiteral;
    pub use crate::shard::ShardParam;
    pub use crate::shard::ShardSet;
    pub use crate::shard::StaticShard;

    pub use crate::shard::AlternativeType as Alt;
    pub use crate::shard::ExternType as Ext;
    pub use crate::shard::LiteralType as Lit;
    pub use crate::shard::OptionType as Opt;
    pub use crate::shard::SequenceType as Seq;
    pub use crate::shard::SetType as Set;
    pub use crate::shard::VecType as Vec;
}

mod shard;

use std::marker::PhantomData;

use crate::shard::{ShardDataType, StaticShard};

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
        let data = S::Data::DATA;

        Self {
            _shard: PhantomData,
        }
    }

    pub fn parse<'i>(&self, input: &'i str) -> S {
        todo!()
    }
}
