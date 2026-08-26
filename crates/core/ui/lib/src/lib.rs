pub trait Shard {}

// use std::ops::RangeInclusive;

// pub trait Shard {
//     fn table() -> &'static [Rule];
// }

// #[derive(Debug)]
// pub struct Rule {
//     literal: Literal,
//     action: Action,
// }

// #[derive(Debug)]
// pub enum Literal {
//     Sequence(&'static str),
//     Set(&'static [RangeInclusive<char>]),
// }

// #[derive(Debug, Clone, Copy)]
// pub enum Action {
//     Shift(usize),
//     Reduce(usize),
//     Accept,
//     Error,
// }

// pub struct Resolver {}

// impl Resolver {
//     fn new<S>(shard: S) -> Self
//     where
//         S: Shard,
//     {
//         Self {}
//     }
// }
