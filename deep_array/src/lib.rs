//! deep_array provides the [Array] type, which is a n-dimensional array.
#![deny(missing_docs)]
#![feature(array_zip)]

pub mod array;
pub mod error;

pub mod allocation;
// mod arithmetic; // Does not work, deactivated for now
mod prelude;

#[cfg(feature = "debug_allocator")]
mod debug_allocator;

pub use prelude::*;
