/// Internal API for generated code.
/// This module is not intended for direct use.
#[doc(hidden)]
pub mod internal {
    pub use core::marker::PhantomData;
    pub use core::ops::RangeInclusive;
    pub use core::primitive::char;
    pub use core::primitive::str;

    pub use crate::shard::FieldOutput;
    pub use crate::shard::Output;
    pub use crate::shard::ParamOutput;
    pub use crate::shard::Shard;
    pub use crate::shard::ShardDataType;
    pub use crate::shard::ShardField;
    pub use crate::shard::ShardLiteral;
    pub use crate::shard::ShardParam;
    pub use crate::shard::ShardSet;
    pub use crate::shard::StaticShard;

    pub use crate::shard::AlternativeType as Alt;
    pub use crate::shard::CaptureType as Capture;
    pub use crate::shard::ExternType as Ext;
    pub use crate::shard::LiteralType as Lit;
    pub use crate::shard::OptionType as Opt;
    pub use crate::shard::SequenceType as Seq;
    pub use crate::shard::SetType as Set;
    pub use crate::shard::VecType as Vec;
}

mod parser;
mod shard;
mod table;

use core::any::TypeId;
use core::marker::PhantomData;

use crate::shard::{ShardDataType, StaticShard};

pub use parser::ParseError;

pub struct Cluster<S>
where
    S: StaticShard,
{
    table: table::Table,
    _shard: PhantomData<fn() -> S>,
}

impl<S> Cluster<S>
where
    S: StaticShard,
{
    pub fn build() -> Self {
        let table = table::Table::build(TypeId::of::<S::Data>(), S::Data::DATA);

        Self {
            table,
            _shard: PhantomData,
        }
    }

    /// Recognizes the entire input, accepting if any GLR branch succeeds.
    /// Output extraction and mapping are not performed yet.
    pub fn parse(&self, input: &str) -> Result<(), ParseError> {
        self.table.parse(input)
    }
}
