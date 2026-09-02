use std::{any::TypeId, marker::PhantomData, ops::RangeInclusive};

pub type Str = str;

pub trait Shard {
    #[allow(private_bounds)]
    type DATA: ShardDataType;
}

pub(crate) trait ShardDataType: 'static {
    const DATA: &'static ShardData;
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
    const DATA: &'static ShardData = &ShardData::Literal(T::LITERAL);
}

pub trait ShardLiteral: 'static {
    const LITERAL: &'static str;
}

pub struct Set<T, const NEGATED: bool>(PhantomData<fn() -> T>);

impl<T, const NEGATED: bool> ShardDataType for Set<T, NEGATED>
where
    T: ShardSet,
{
    const DATA: &'static ShardData = &ShardData::Set(SetData {
        negated: NEGATED,
        range: T::SET,
    });
}

pub trait ShardSet: 'static {
    const SET: &'static [RangeInclusive<char>];
}

pub struct Option<T>(PhantomData<fn() -> T>);

impl<T> ShardDataType for Option<T>
where
    T: ShardDataType,
{
    const DATA: &'static ShardData = &ShardData::Option(T::DATA);
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
        &ShardData::Extern(TypeId::of::<T::DATA>(), || <T::DATA as ShardDataType>::DATA);
}

#[derive(Debug)]
pub(crate) enum ShardData {
    Literal(&'static str),
    Set(SetData),
    Option(&'static ShardData),
    Vector(VectorData),
    Sequence(&'static [&'static ShardData]),
    Alternative(&'static [&'static ShardData]),
    Extern(TypeId, fn() -> &'static ShardData),
}

#[derive(Debug)]
pub(crate) struct SetData {
    pub negated: bool,
    pub range: &'static [RangeInclusive<char>],
}

#[derive(Debug)]
pub(crate) struct VectorData {
    pub item: &'static ShardData,
    pub min: usize,
    pub max: usize,
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
