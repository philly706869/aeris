use std::{any::TypeId, marker::PhantomData, ops::RangeInclusive};

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
    _shard: PhantomData<fn() -> S>,
}

impl<S> Cluster<S>
where
    S: StaticShard,
{
    pub const fn build() -> Self {
        let data = <<S::Static as ShardStatic>::Data as ShardExt>::DATA;
        Self {
            _shard: PhantomData,
        }
    }

    pub fn parse<'i>(&self, input: &'i str) -> S {
        todo!()
    }
}

pub trait Shard {
    type Static: ShardStatic;
}

pub trait ShardStatic: 'static {
    #[allow(private_bounds)]
    type Data: ShardExt;
}

pub trait StaticShard: Shard {}

trait ShardExt {
    const DATA: &'static ShardData;
}

pub trait ShardLiteral {
    const LITERAL: &'static str;
}

pub trait ShardSet {
    const SET: &'static [RangeInclusive<char>];
}

pub mod shards {
    use std::{any::TypeId, marker::PhantomData};

    use super::{Shard, ShardData, ShardExt, ShardLiteral, ShardSet, ShardStatic};

    pub struct Literal<T>(PhantomData<fn() -> T>);

    impl<T> ShardExt for Literal<T>
    where
        T: ShardLiteral,
    {
        const DATA: &'static ShardData = &ShardData::Literal(T::LITERAL);
    }

    pub struct Set<T, const NEGATED: bool>(PhantomData<fn() -> T>);

    impl<T, const NEGATED: bool> ShardExt for Set<T, NEGATED>
    where
        T: ShardSet,
    {
        const DATA: &'static ShardData = &ShardData::Set {
            negated: NEGATED,
            range: T::SET,
        };
    }

    pub struct Option<T>(PhantomData<fn() -> T>);

    impl<T> ShardExt for Option<T>
    where
        T: ShardExt,
    {
        const DATA: &'static ShardData = &ShardData::Option(T::DATA);
    }

    pub struct Vec<T, const MIN: usize, const MAX: usize>(PhantomData<fn() -> T>);

    impl<T, const MIN: usize, const MAX: usize> ShardExt for Vec<T, MIN, MAX>
    where
        T: ShardExt,
    {
        const DATA: &'static ShardData = &ShardData::Vector {
            item: <T as ShardExt>::DATA,
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
            impl<$($t),*> ShardExt for Sequence<($($t,)*)>
            where
                $($t: ShardExt),*
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
            impl<$($t),*> ShardExt for Alternative<($($t,)*)>
            where
                $($t: ShardExt),*
            {
                const DATA: &'static ShardData = &ShardData::Alternative(&[$($t::DATA),*]);
            }
        }
    }

    impl_shard_ext_for_alternative!();

    pub struct Extern<T>(PhantomData<fn() -> T>);

    impl<T> ShardExt for Extern<T>
    where
        T: ShardStatic,
    {
        const DATA: &'static ShardData =
            &ShardData::Extern(TypeId::of::<T>(), <T::Data as ShardExt>::DATA);
    }
}

#[derive(Debug)]
enum ShardData {
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
