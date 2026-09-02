use std::{any::TypeId, marker::PhantomData, ops::RangeInclusive};

pub type Str = str;

pub trait Shard {
    type TUID: 'static;
    #[allow(private_bounds)]
    type DATA: ShardDataType;
}

pub trait StaticShard: Shard {}

pub(crate) trait ShardDataType {
    const DATA: &'static ShardData;
}

pub trait ShardParam: ShardDataType {}

impl<T> ShardParam for T where T: ShardDataType {}

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
    const DATA: &'static ShardData = &ShardData::Set {
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
    const DATA: &'static ShardData = &ShardData::Option(T::DATA);
}

pub struct Vec<T, const MIN: usize, const MAX: usize>(PhantomData<fn() -> T>);

impl<T, const MIN: usize, const MAX: usize> ShardDataType for Vec<T, MIN, MAX>
where
    T: ShardDataType,
{
    const DATA: &'static ShardData = &ShardData::Vector {
        item: <T as ShardDataType>::DATA,
        min: MIN,
        max: MAX,
    };
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
    T: Shard,
{
    const DATA: &'static ShardData =
        &ShardData::Extern(TypeId::of::<T::TUID>(), <T::DATA as ShardDataType>::DATA);
}

#[derive(Debug)]
pub(crate) enum ShardData {
    Literal(&'static str),
    Set {
        negated: bool,
        range: &'static [RangeInclusive<char>],
    },
    Option(&'static ShardData),
    Vector {
        item: &'static ShardData,
        min: usize,
        max: usize,
    },
    Sequence(&'static [&'static ShardData]),
    Alternative(&'static [&'static ShardData]),
    Extern(TypeId, &'static ShardData),
}
