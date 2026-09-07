use std::{any::TypeId, marker::PhantomData, ops::RangeInclusive};

pub type Str = str;

pub trait Shard {
    #[allow(private_bounds)]
    type Data: ShardDataType;
}

pub(crate) trait ShardDataType: 'static {
    type Data: Into<ShardData>;
    const DATA: &'static Self::Data;
}

#[allow(private_bounds)]
pub trait ShardParam: ShardDataType {}

impl<T> ShardParam for T where T: ShardDataType {}

pub trait StaticShard: Shard {}

pub struct Literal<T>(PhantomData<fn() -> T>);

impl<T> ShardDataType for Literal<T>
where
    T: ShardLiteral,
{
    type Data = LiteralData;
    const DATA: &'static LiteralData = &LiteralData { text: T::LITERAL };
}

pub trait ShardLiteral: 'static {
    const LITERAL: &'static str;
}

pub struct Set<const NEGATED: bool, T>(PhantomData<fn() -> T>);

impl<const NEGATED: bool, T> ShardDataType for Set<NEGATED, T>
where
    T: ShardSet,
{
    type Data = SetData;
    const DATA: &'static SetData = &SetData {
        negated: NEGATED,
        range: T::SET,
    };
}

pub trait ShardSet: 'static {
    const SET: &'static [RangeInclusive<char>];
}

pub struct Option<T>(PhantomData<fn() -> T>);

impl<T> ShardDataType for Option<T>
where
    T: ShardDataType,
{
    type Data = OptionData;
    const DATA: &'static OptionData = &OptionData {
        item: T::DATA.into(),
    };
}

pub struct Vec<T, const MIN: usize, const MAX: usize>(PhantomData<fn() -> T>);

impl<T, const MIN: usize, const MAX: usize> ShardDataType for Vec<T, MIN, MAX>
where
    T: ShardDataType,
{
    const DATA: &'static ShardData = &ShardData::Vector(VectorData {
        item: <T as ShardDataType>::DATA,
        min: MIN,
        max: MAX,
    });
}

pub struct Sequence<T>(PhantomData<fn() -> T>);

macro_rules! impl_shard_ext_for_sequence {
    () => {
        impl_shard_ext_for_sequence![@for A B C D E F G H I J K L M N O P Q R S T U V W X Y Z];
    };
    [@for $first_t:ident $($t:ident)*] => {
        impl_shard_ext_for_sequence![@for $($t)*];
        impl_shard_ext_for_sequence![@impl $first_t $($t)*];
    };
    [@for] => {
        impl_shard_ext_for_sequence![@impl];
    };
    [@impl $($t:ident)*] => {
        impl<$($t),*> ShardDataType for Sequence<($($t,)*)>
        where
            $($t: ShardDataType),*
        {
            const DATA: &'static ShardData = &ShardData::Sequence(&[$($t::DATA),*]);
        }
    }
}

impl_shard_ext_for_sequence!();

pub struct Alternative<T>(PhantomData<fn() -> T>);

macro_rules! impl_shard_ext_for_alternative {
    () => {
        impl_shard_ext_for_alternative![@for A B C D E F G H I J K L M N O P Q R S T U V W X Y Z];
    };
    [@for $first_t:ident $($t:ident)*] => {
        impl_shard_ext_for_alternative![@for $($t)*];
        impl_shard_ext_for_alternative![@impl $first_t $($t)*];
    };
    [@for] => {
        impl_shard_ext_for_alternative![@impl];
    };
    [@impl $($t:ident)*] => {
        impl<$($t),*> ShardDataType for Alternative<($($t,)*)>
        where
            $($t: ShardDataType),*
        {
            const DATA: &'static ShardData = &ShardData::Alternative(&[$($t::DATA),*]);
        }
    }
}

impl_shard_ext_for_alternative!();

pub struct Extern<T>(PhantomData<fn() -> T>);

impl<T> ShardDataType for Extern<T>
where
    T: Shard + 'static,
{
    const DATA: &'static ShardData =
        &ShardData::Extern(TypeId::of::<T::Data>(), || <T::Data as ShardDataType>::DATA);
}

#[derive(Debug)]
pub(crate) enum ShardData {
    Literal(LiteralData),
    Set(SetData),
    Option(OptionData),
    Vector(VectorData),
    Sequence(SequenceData),
    Alternative(AlternativeData),
    Extern(ExternData),
}

macro_rules! impl_shard_data_from {
    ($($ty:ty => $variant:ident,)*) => {
        $(
            impl From<$ty> for ShardData {
                fn from(value: $ty) -> Self {
                    Self::$variant(value)
                }
            }
        )*
    };
}

impl_shard_data_from! {
    LiteralData => Literal,
    SetData => Set,
    OptionData => Option,
    VectorData => Vector,
    SequenceData => Sequence,
    AlternativeData => Alternative,
    ExternData => Extern,
}

#[derive(Debug)]
pub(crate) struct LiteralData {
    pub text: &'static str,
}

#[derive(Debug)]
pub(crate) struct SetData {
    pub negated: bool,
    pub range: &'static [RangeInclusive<char>],
}

#[derive(Debug)]
pub(crate) struct OptionData {
    pub item: &'static ShardData,
}

#[derive(Debug)]
pub(crate) struct VectorData {
    pub item: &'static ShardData,
    pub min: usize,
    pub max: usize,
}

#[derive(Debug)]
pub(crate) struct SequenceData {
    pub items: &'static [&'static ShardData],
}

#[derive(Debug)]
pub(crate) struct AlternativeData {
    pub items: &'static [&'static ShardData],
}

#[derive(Debug)]
pub(crate) struct ExternData {
    pub id: TypeId,
    pub reference: fn() -> &'static ShardData,
}

fn normalize_set(set: &SetData) -> std::vec::Vec<RangeInclusive<char>> {
    use std::vec::Vec;
    let mut ranges = Vec::from(set.range);
    ranges.sort_unstable_by_key(|r| (*r.start(), *r.end()));
    let mut normalized: Vec<RangeInclusive<char>> = Vec::with_capacity(ranges.len());
    for range in ranges {
        let start = *range.start();
        let end = *range.end();
        match normalized.last_mut() {
            Some(last) => {
                let last_end = *last.end();
                if start as u32 <= last_end as u32 + 1 && end > last_end {
                    *last = *last.start()..=end;
                } else {
                    normalized.push(range);
                }
            }
            None => normalized.push(range),
        }
    }
    normalized
}
