use std::any::TypeId;

use crate::Bencher;

mod meta;
mod tree;

pub use meta::{EntryLocation, EntryMeta};
pub(crate) use tree::EntryTree;

/// Benchmark entries generated by `#[divan::bench]`.
///
/// Note: generic-type benchmark entries are instead stored in `GROUP_ENTRIES`
/// in `generic_benches`.
#[linkme::distributed_slice]
pub static BENCH_ENTRIES: [BenchEntry] = [..];

/// Group entries generated by `#[divan::bench_group]`.
#[linkme::distributed_slice]
pub static GROUP_ENTRIES: [GroupEntry] = [..];

/// Compile-time entry for a benchmark, generated by `#[divan::bench]`.
pub struct BenchEntry {
    /// Entry metadata.
    pub meta: EntryMeta,

    /// The benchmarking function.
    pub bench: fn(Bencher),
}

/// Compile-time entry for a benchmark group, generated by
/// `#[divan::bench_group]` or a generic-type `#[divan::bench]`.
pub struct GroupEntry {
    /// Entry metadata.
    pub meta: EntryMeta,

    /// Generic `#[divan::bench]` entries.
    pub generic_benches: Option<&'static [GenericBenchEntry]>,
}

/// Compile-time entry for a generic-type benchmark, generated by
/// `#[divan::bench]`.
///
/// Unlike `BenchEntry`, this is for a specific generic type.
pub struct GenericBenchEntry {
    /// The associated group, for entry metadata.
    pub group: &'static GroupEntry,

    /// The benchmarking function.
    pub bench: fn(Bencher),

    /// [`std::any::type_name`].
    pub get_type_name: fn() -> &'static str,

    /// [`std::any::TypeId::of`].
    pub get_type_id: fn() -> TypeId,
}

impl GenericBenchEntry {
    #[inline]
    pub(crate) fn raw_name(&self) -> &'static str {
        (self.get_type_name)()
    }

    pub(crate) fn display_name(&self) -> &'static str {
        let mut type_name = (self.get_type_name)();

        // Remove module components in type name.
        while let Some((prev, next)) = type_name.split_once("::") {
            // Do not go past generic type boundary.
            if prev.contains('<') {
                break;
            }
            type_name = next;
        }

        type_name
    }
}

/// `BenchEntry` or `GenericBenchEntry`.
#[derive(Clone, Copy)]
pub(crate) enum AnyBenchEntry<'a> {
    Bench(&'a BenchEntry),
    GenericBench(&'a GenericBenchEntry),
}

impl<'a> AnyBenchEntry<'a> {
    #[inline]
    pub fn bench(self, bencher: Bencher) {
        match self {
            Self::Bench(BenchEntry { bench, .. })
            | Self::GenericBench(GenericBenchEntry { bench, .. }) => bench(bencher),
        }
    }

    #[inline]
    pub fn meta(self) -> &'a EntryMeta {
        match self {
            Self::Bench(entry) => &entry.meta,
            Self::GenericBench(entry) => &entry.group.meta,
        }
    }

    #[inline]
    pub fn raw_name(self) -> &'a str {
        match self {
            Self::Bench(entry) => entry.meta.raw_name,
            Self::GenericBench(entry) => entry.raw_name(),
        }
    }

    #[inline]
    pub fn display_name(self) -> &'a str {
        match self {
            Self::Bench(entry) => entry.meta.display_name,
            Self::GenericBench(entry) => entry.display_name(),
        }
    }
}
