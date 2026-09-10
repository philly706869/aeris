use core::marker::PhantomData;

mod parser;
mod shard;
mod table;

pub use parser::ParseError;
pub use shard::internal;

use core::any::TypeId;
use shard::{ShardCore, StaticShard};

pub struct Cluster<S>
where
    S: StaticShard,
{
    _shard: PhantomData<fn() -> S>,
    table: table::Table,
}

impl<S> Cluster<S>
where
    S: StaticShard,
{
    pub fn build() -> Self {
        Self {
            _shard: PhantomData,
            table: table::Table::build(TypeId::of::<S::Core>(), <S::Core as ShardCore>::DATA),
        }
    }

    /// Recognizes the entire input, accepting if any GLR branch succeeds.
    /// Output extraction and mapping are not performed yet.
    pub fn parse(&self, input: &str) -> Result<(), ParseError> {
        self.table.parse(input)
    }
}
