use std::any::TypeId;

use crate::bench::Context;

/// Compile-time benchmark entry generated by `#[divan::bench]`.
pub struct Entry {
    /// The benchmarked function's fully-qualified path.
    pub path: &'static str,

    /// The file where the benchmarked function was defined.
    pub file: &'static str,

    /// The line where the benchmarked function was defined.
    pub line: u32,

    /// The benchmarking loop.
    pub bench_loop: fn(&mut Context),

    /// Returns the globally unique ID of a benchmarked function.
    ///
    /// The Rust reference guarantees that [each function item has a unique
    /// type](https://doc.rust-lang.org/1.70.0/reference/types/function-item.html):
    /// > Because the function item type explicitly identifies the function, the
    /// > item types of different functions - different items, or the same item
    /// > with different generics - are distinct, and mixing them will create a
    /// > type error.
    ///
    /// This ID should be stable across runs if builds are reproducible, but it
    /// is not stable between different compiler versions.
    ///
    /// In the future this field should just be `TypeId` instead of a function,
    /// but `TypeId::of` is [not yet usable in `const`](https://github.com/rust-lang/rust/issues/77125).
    pub get_id: fn() -> TypeId,
}

/// Entries generated by `#[divan::bench]`.
#[linkme::distributed_slice]
pub static ENTRIES: [Entry] = [..];
