use std::{any::TypeId, marker::PhantomData, ops::RangeInclusive};

///
#[macro_export]
macro_rules! x {
    () => {};
}

#[macro_export]
macro_rules! cluster {
    () => {};
}

pub struct Cluster<S>
where
    S: StaticShard,
{
    _shard: PhantomData<S>,
}

impl<S> Cluster<S>
where
    S: StaticShard,
{
    pub const fn build() -> Self {
        Self {
            _shard: PhantomData,
        }
    }

    pub fn parse<'i>(&self, input: &'i str) -> S {
        todo!()
    }
}

pub trait Shard {
    const TUID: TypeId;
}

pub trait StaticShard: Shard {
    const DATA: &'static ShardData;
}

pub trait DynamicShard: Shard {}

pub enum ShardData {
    Literal(&'static str),
    Set(bool, &'static [RangeInclusive<char>]),
    Sequence(&'static [ShardData]),
    Alternative(&'static [ShardData]),
    Vector(&'static ShardData, usize, usize),
    Option(&'static ShardData),
}

mod sealed {
    pub trait Sealed {}
}

use sealed::Sealed;

pub trait ShardEntry
where
    Self: Sealed,
{
}

impl<'i> Sealed for &'i str {}
impl<'i> ShardEntry for &'i str {}

impl<T> Sealed for T where T: Shard {}
impl<T> ShardEntry for T where T: Shard {}

// impl<T> Sealed for Box<T> where T: ShardEntry {}
// impl<T> ShardEntry for Box<T> where T: ShardEntry {}

macro_rules! impl_shard_entry_for_tuple {
    () => {
        impl_shard_entry_for_tuple![@for A B C D E F G H I J K L M N O P Q R S T U V W X Y Z];
    };
    [@for $first_t:ident $($t:ident)*] => {
        impl_shard_entry_for_tuple![@for $($t)*];
        impl_shard_entry_for_tuple![@impl $first_t $($t)*];
    };
    [@for] => {
        impl_shard_entry_for_tuple![@impl];
    };
    [@impl $($t:ident)*] => {
        impl<$($t),*> Sealed for ($($t,)*)
        where
            $($t: ShardEntry),*
        {}
        impl<$($t),*> ShardEntry for ($($t,)*)
        where
            $($t: ShardEntry),*
        {}
    }
}

impl_shard_entry_for_tuple!();
