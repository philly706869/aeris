use core::any::TypeId;
use core::marker::PhantomData;

mod parser;
mod shard;
mod table;

pub use parser::ParseError;
pub use shard::internal;

use shard::{ShardDataType, StaticShard};

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
        // let table = table::Table::build(TypeId::of::<S::Data>(), S::Data::DATA);
        let table = todo!();

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
