use core::{any::TypeId, fmt::Debug, ops::RangeInclusive};

pub mod internal {
    pub use crate::cluster::{ExtractError, MappingNode};
    pub use crate::shard::Shard;
    pub use crate::shard::ShardClosure;
    pub use crate::shard::ShardClosureForward;
    pub use crate::shard::ShardCore;
    pub use crate::shard::ShardData;
    pub use crate::shard::ShardField;
    pub use crate::shard::StaticShard;
}

pub trait StaticShard: Shard {}

pub trait Shard: 'static {
    type Core: ShardCore;
}

pub trait ShardCore: 'static {
    type Output<'i>: Debug;
    const DATA: &'static ShardData;
    fn map<'i>(
        _node: crate::cluster::MappingNode<'_, 'i>,
    ) -> Result<Self::Output<'i>, crate::cluster::ExtractError> {
        Err(crate::cluster::ExtractError::InvalidMapping)
    }
}

pub type ShardField<'i, T> = <<T as Shard>::Core as ShardCore>::Output<'i>;

pub type ShardClosure<T, const INDEX: usize> =
    <<T as Shard>::Core as ShardClosureForward<INDEX>>::Closure;

pub trait ShardClosureForward<const INDEX: usize>: ShardCore {
    type Closure: Shard;
}

#[derive(Debug)]
pub struct ShardData {
    pub(crate) kind: ShardDataKind,
}

impl ShardData {
    const fn new(kind: ShardDataKind) -> Self {
        Self { kind }
    }

    pub const fn literal(text: &'static str) -> Self {
        Self::new(ShardDataKind::Literal(LiteralData::new(text)))
    }

    pub const fn set(negated: bool, range: &'static [RangeInclusive<char>]) -> Self {
        Self::new(ShardDataKind::Set(SetData::new(negated, range)))
    }

    pub const fn option(item: &'static ShardData) -> Self {
        Self::new(ShardDataKind::Option(OptionData::new(item, false)))
    }

    pub const fn option_lazy(item: &'static ShardData) -> Self {
        Self::new(ShardDataKind::Option(OptionData::new(item, true)))
    }

    pub const fn vec(item: &'static ShardData, min: usize, max: Option<usize>) -> Self {
        Self::new(ShardDataKind::Vec(VecData::new(item, min, max, false)))
    }

    pub const fn vec_lazy(item: &'static ShardData, min: usize, max: Option<usize>) -> Self {
        Self::new(ShardDataKind::Vec(VecData::new(item, min, max, true)))
    }

    pub const fn sequence(items: &'static [&'static ShardData]) -> Self {
        Self::new(ShardDataKind::Sequence(SequenceData::new(items)))
    }

    pub const fn alternative(items: &'static [&'static ShardData]) -> Self {
        Self::new(ShardDataKind::Alternative(AlternativeData::new(items)))
    }

    pub const fn reference<T>() -> Self
    where
        T: Shard,
    {
        Self::new(ShardDataKind::Reference(ReferenceData::new::<T>()))
    }
}

#[derive(Debug)]
pub enum ShardDataKind {
    Literal(LiteralData),
    Set(SetData),
    Option(OptionData),
    Vec(VecData),
    Sequence(SequenceData),
    Alternative(AlternativeData),
    Reference(ReferenceData),
}

#[derive(Debug)]
pub struct LiteralData {
    pub text: &'static str,
}

impl LiteralData {
    pub const fn new(text: &'static str) -> Self {
        Self { text }
    }
}

#[derive(Debug)]
pub struct SetData {
    pub negated: bool,
    pub range: &'static [RangeInclusive<char>],
}

impl SetData {
    pub const fn new(negated: bool, range: &'static [RangeInclusive<char>]) -> Self {
        Self { negated, range }
    }
}

#[derive(Debug)]
pub struct OptionData {
    pub item: &'static ShardData,
    pub lazy: bool,
}

impl OptionData {
    pub const fn new(item: &'static ShardData, lazy: bool) -> Self {
        Self { item, lazy }
    }
}

#[derive(Debug)]
pub struct VecData {
    pub item: &'static ShardData,
    pub min: usize,
    pub max: Option<usize>,
    pub lazy: bool,
}

impl VecData {
    pub const fn new(item: &'static ShardData, min: usize, max: Option<usize>, lazy: bool) -> Self {
        Self {
            item,
            min,
            max,
            lazy,
        }
    }
}

#[derive(Debug)]
pub struct SequenceData {
    pub items: &'static [&'static ShardData],
}

impl SequenceData {
    pub const fn new(items: &'static [&'static ShardData]) -> Self {
        Self { items }
    }
}

#[derive(Debug)]
pub struct AlternativeData {
    pub items: &'static [&'static ShardData],
}

impl AlternativeData {
    pub const fn new(items: &'static [&'static ShardData]) -> Self {
        Self { items }
    }
}

#[derive(Debug)]
pub struct ReferenceData {
    pub id: TypeId,
    pub reference: fn() -> &'static ShardData,
}

impl ReferenceData {
    pub const fn new<T>() -> Self
    where
        T: Shard,
    {
        Self {
            id: TypeId::of::<T::Core>(),
            reference: || T::Core::DATA,
        }
    }
}
