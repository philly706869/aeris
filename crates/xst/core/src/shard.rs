use core::{any::TypeId, fmt::Debug, marker::PhantomData, ops::RangeInclusive};

/// Internal API for generated code.
/// This module is not intended for direct use.
#[doc(hidden)]
pub mod internal {
    pub use core::marker::PhantomData;
    pub use core::ops::RangeInclusive;
    pub use core::primitive::char;
    pub use core::primitive::str;

    pub use super::FieldOutput;
    pub use super::Output;
    pub use super::ParamOutput;
    pub use super::Shard;
    pub use super::ShardCore;
    pub use super::ShardData;
    pub use super::ShardDataType;
    pub use super::ShardField;
    pub use super::ShardLiteral;
    pub use super::ShardParam;
    pub use super::ShardSet;
    pub use super::StaticShard;

    pub use super::AlternativeType as Alt;
    pub use super::CaptureType as Capture;
    pub use super::ExternType as Ext;
    pub use super::LiteralType as Lit;
    pub use super::OptionType as Opt;
    pub use super::SequenceType as Seq;
    pub use super::SetType as Set;
    pub use super::VecType as Vec;
}

pub trait StaticShard: Shard {}

pub trait Shard {
    type Core: ShardCore;
}

pub trait ShardCore: 'static {
    type Data: ShardDataType;
    type Output<'i>: Debug;
}

pub type Output<'i, S> = <<S as Shard>::Core as ShardCore>::Output<'i>;

/// Output of a generated binding field. Implementations live in the binding's
/// anonymous const so grammar-only helper names never enter the module scope.
/// FIELD is the zero-based source field index (before repeated-name grouping).
/// For enum bindings, FIELD is the variant index; each variant contains one shard.
pub trait ShardField<const INDEX: usize>: Shard {
    type Output<'i>: Debug;
}

pub type FieldOutput<'i, S, const INDEX: usize> = <S as ShardField<INDEX>>::Output<'i>;

pub trait ShardDataType: 'static {
    const DATA: &'static ShardData;
}

pub trait ShardParam: ShardDataType {
    type Output<'i>: core::fmt::Debug;
}

pub type ParamOutput<'i, T> = <T as ShardParam>::Output<'i>;

impl<T: ShardLiteral> ShardParam for LiteralType<T> {
    type Output<'i> = &'i str;
}

impl<const NEGATED: bool, T: ShardSet> ShardParam for SetType<NEGATED, T> {
    type Output<'i> = &'i str;
}

impl<T> ShardParam for CoreType<T>
where
    T: ShardCore,
    for<'i> T::Output<'i>: core::fmt::Debug,
{
    type Output<'i> = T::Output<'i>;
}

impl<T: ShardParam> ShardParam for OptionType<T> {
    type Output<'i> = Option<T::Output<'i>>;
}

impl<T: ShardParam, const MIN: usize, const MAX: usize> ShardParam for VecType<T, MIN, MAX> {
    type Output<'i> = Vec<T::Output<'i>>;
}

/// An inline x! pattern captures its entire matched slice, regardless of its
/// internal grammar (sequence, alternative, repetition, or shard references).
pub struct CaptureType<T>(PhantomData<fn() -> T>);

impl<T> ShardDataType for CaptureType<T>
where
    T: ShardDataType,
{
    const DATA: &'static ShardData = T::DATA;
}

impl<T> ShardParam for CaptureType<T>
where
    T: ShardDataType,
{
    type Output<'i> = &'i str;
}

pub struct LiteralType<T>(PhantomData<fn() -> T>);

impl<T> ShardDataType for LiteralType<T>
where
    T: ShardLiteral,
{
    const DATA: &'static ShardData = &ShardData::Literal(LiteralData { text: T::LITERAL });
}

pub trait ShardLiteral: 'static {
    const LITERAL: &'static str;
}

pub struct SetType<const NEGATED: bool, T>(PhantomData<fn() -> T>);

impl<const NEGATED: bool, T> ShardDataType for SetType<NEGATED, T>
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

pub struct OptionType<T>(PhantomData<fn() -> T>);

impl<T> ShardDataType for OptionType<T>
where
    T: ShardDataType,
{
    const DATA: &'static ShardData = &ShardData::Option(OptionData { item: T::DATA });
}

pub struct VecType<T, const MIN: usize, const MAX: usize>(PhantomData<fn() -> T>);

impl<T, const MIN: usize, const MAX: usize> ShardDataType for VecType<T, MIN, MAX>
where
    T: ShardDataType,
{
    const DATA: &'static ShardData = &ShardData::Vec(VecData {
        item: T::DATA,
        min: MIN,
        max: MAX,
    });
}

pub struct SequenceType<T>(PhantomData<fn() -> T>);

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
        impl<$($t),*> ShardParam for SequenceType<($($t,)*)>
        where
            for<'i> ($($t::Output<'i>,)*): core::fmt::Debug,
            $($t: ShardParam),*
        {
            type Output<'i> = ($($t::Output<'i>,)*);
        }

        impl<$($t),*> ShardDataType for SequenceType<($($t,)*)>
        where
            $($t: ShardDataType),*
        {
            const DATA: &'static ShardData = &ShardData::Sequence(SequenceData {
                items: &[$($t::DATA),*],
            });
        }
    }
}

impl_shard_ext_for_sequence!();

pub struct AlternativeType<T>(PhantomData<fn() -> T>);

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
        impl<$($t),*> ShardDataType for AlternativeType<($($t,)*)>
        where
            $($t: ShardDataType),*
        {
            const DATA: &'static ShardData = &ShardData::Alternative(AlternativeData {
                items: &[$($t::DATA),*],
            });
        }
    }
}

impl_shard_ext_for_alternative!();

/// Resolve a borrowed shard to its static core. Only the core is stored in
/// the grammar descriptor; neither its identity nor its lifetime depends on S.
/// This alias does not bypass Rust's visibility checks on a source reference.
pub type ExternType<T> = CoreType<<T as Shard>::Core>;

pub struct CoreType<T>(PhantomData<fn() -> T>);

impl<T> ShardDataType for CoreType<T>
where
    T: ShardCore,
{
    const DATA: &'static ShardData = &ShardData::Extern(ExternData {
        id: TypeId::of::<T>(),
        reference: || <T::Data as ShardDataType>::DATA,
    });
}

#[derive(Debug)]
pub enum ShardData {
    Literal(LiteralData),
    Set(SetData),
    Option(OptionData),
    Vec(VecData),
    Sequence(SequenceData),
    Alternative(AlternativeData),
    Extern(ExternData),
}

#[derive(Debug)]
pub struct LiteralData {
    pub text: &'static str,
}

#[derive(Debug)]
pub struct SetData {
    pub negated: bool,
    pub range: &'static [RangeInclusive<char>],
}

#[derive(Debug)]
pub struct OptionData {
    pub item: &'static ShardData,
}

#[derive(Debug)]
pub struct VecData {
    pub item: &'static ShardData,
    pub min: usize,
    pub max: usize,
}

#[derive(Debug)]
pub struct SequenceData {
    pub items: &'static [&'static ShardData],
}

#[derive(Debug)]
pub struct AlternativeData {
    pub items: &'static [&'static ShardData],
}

#[derive(Debug)]
pub struct ExternData {
    pub id: TypeId,
    pub reference: fn() -> &'static ShardData,
}

fn normalize_set(set: &SetData) -> Vec<RangeInclusive<char>> {
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
