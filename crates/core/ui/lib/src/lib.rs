use std::{any::TypeId, ops::RangeInclusive};

pub trait Shard {
    const TUID: TypeId;
    const DATA: &'static ShardData;
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

fn test<S>()
where
    S: ShardEntry,
{
}

impl<T> Sealed for T where T: Shard {}
impl<T> ShardEntry for T where T: Shard {}

impl<'t> Sealed for &'t str {}
impl<'t> ShardEntry for &'t str {}

macro_rules! impl_shard_entry_for_tuple {
    { $($generic:ident),* } => {
        impl<$($generic),*> Sealed for ($($generic,)*) where $($generic: ShardEntry),* {}
        impl<$($generic),*> ShardEntry for ($($generic,)*) where $($generic: ShardEntry),* {}
    };
}

impl_shard_entry_for_tuple! {}
impl_shard_entry_for_tuple! { A }
impl_shard_entry_for_tuple! { A, B }
impl_shard_entry_for_tuple! { A, B, C }
impl_shard_entry_for_tuple! { A, B, C, D }
impl_shard_entry_for_tuple! { A, B, C, D, E }
impl_shard_entry_for_tuple! { A, B, C, D, E, F }
impl_shard_entry_for_tuple! { A, B, C, D, E, F, G }
impl_shard_entry_for_tuple! { A, B, C, D, E, F, G, H }
impl_shard_entry_for_tuple! { A, B, C, D, E, F, G, H, I }
impl_shard_entry_for_tuple! { A, B, C, D, E, F, G, H, I, J }
impl_shard_entry_for_tuple! { A, B, C, D, E, F, G, H, I, J, K }
impl_shard_entry_for_tuple! { A, B, C, D, E, F, G, H, I, J, K, L }
impl_shard_entry_for_tuple! { A, B, C, D, E, F, G, H, I, J, K, L, M }
impl_shard_entry_for_tuple! { A, B, C, D, E, F, G, H, I, J, K, L, M, N }
impl_shard_entry_for_tuple! { A, B, C, D, E, F, G, H, I, J, K, L, M, N, O }
impl_shard_entry_for_tuple! { A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P }
impl_shard_entry_for_tuple! { A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q }
impl_shard_entry_for_tuple! { A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R }
impl_shard_entry_for_tuple! { A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S }
impl_shard_entry_for_tuple! { A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T }
impl_shard_entry_for_tuple! { A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U }
impl_shard_entry_for_tuple! { A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V }
impl_shard_entry_for_tuple! { A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W }
impl_shard_entry_for_tuple! { A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X }
impl_shard_entry_for_tuple! { A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y }
impl_shard_entry_for_tuple! { A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z }

pub enum ShardData {
    Literal(&'static str),
    Set(bool, Vec<RangeInclusive<char>>),
    Sequence(Vec<ShardData>),
    Alternative(Vec<ShardData>),
}

pub const fn build<S>()
where
    S: Shard,
{
}
