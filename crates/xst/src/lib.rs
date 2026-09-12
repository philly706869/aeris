extern crate alloc;

pub use xst_core::public::*;
pub use xst_proc::shard;

/// Internal API for generated code.
/// This module is not intended for direct use.
#[doc(hidden)]
pub mod internal {
    pub mod fmt {
        pub use core::fmt::Debug;
        pub use core::fmt::Formatter;
        pub use core::fmt::Result;
    }

    pub use core::marker::PhantomData;
    pub use core::ops::RangeInclusive;
    pub use core::option::Option;
    pub use core::primitive::char;
    pub use core::primitive::str;

    pub use alloc::boxed::Box;
    pub use alloc::vec::Vec;

    pub use xst_core::internal::*;

    pub mod rune {
        pub use xst_proc::x;
        pub use xst_proc::xbox;
        pub use xst_proc::xopt;
        pub use xst_proc::xoptz;
        pub use xst_proc::xvec;
        pub use xst_proc::xvecz;
    }
}
