use std::any::Any;

use crate::counter::{IntoCounter, Items};

/// The largest unsigned integer usable by counters provided by this crate.
///
/// If `usize < u64`, this is a type alias to `u64`. Otherwise, it is a type
/// alias to `usize`.
pub type MaxCountUInt = condtype::CondType<{ std::mem::size_of::<usize>() < 8 }, u64, usize>;

/// `u8`-`u64` and `usize`.
///
/// We deliberately do not implement this trait for `u128` to make it
/// impossible† to overflow `u128` when summing counts for averaging.
///
/// †When `usize` is larger than `u64`, it becomes possible to overflow `u128`.
/// In this case, Divan assumes
pub trait CountUInt: Copy + Any {
    fn into_max_uint(self) -> MaxCountUInt;
}

macro_rules! impl_uint {
    ($($i:ty),+) => {
        $(impl CountUInt for $i {
            #[inline]
            fn into_max_uint(self) -> MaxCountUInt {
                self as _
            }
        })+

        $(impl IntoCounter for $i {
            type Counter = Items<$i>;

            #[inline]
            fn into_counter(self) -> Items<$i> {
                Items(self)
            }
        })+
    };
}

// These types must be losslessly convertible to `MaxCountUInt`.
impl_uint!(u8, u16, u32, u64, usize);