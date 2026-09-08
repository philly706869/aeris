pub type Str = str;

pub use std::marker::PhantomData;

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
