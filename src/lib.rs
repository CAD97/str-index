#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

use core::u32;

mod convert;
mod fmt;
mod ops;
#[cfg(feature = "serde")]
mod serde;

/// An index into a string.
///
/// The index is stored as a 32 bit integer,
/// assuming we only deal with text shorter than 4 GiB.
#[derive(Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct StrIndex {
    raw: u32,
}

impl StrIndex {
    /// Index equal to the string length of this `char`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use str_index::*;
    /// assert_eq!(
    ///     StrIndex::from_char_len('😂'),
    ///     StrIndex::from(4),
    /// );
    /// ```
    pub fn from_char_len(c: char) -> Self {
        StrIndex::from(c.len_utf8() as u32)
    }

    /// Index equal to the length of this string.
    ///
    /// # Examples
    ///
    /// ```
    /// # use str_index::*;
    /// assert_eq!(
    ///     StrIndex::from_str_len("メカジキ"),
    ///     StrIndex::from(12),
    /// );
    /// ```
    pub fn from_str_len(s: &str) -> Self {
        assert!(s.len() < u32::MAX as usize, "string index too large");
        StrIndex {
            raw: s.len() as u32,
        }
    }

    /// This index as a raw `usize`.
    pub fn to_usize(self) -> usize {
        self.into()
    }

    /// Checked integer addition.
    pub fn checked_add(self, rhs: Self) -> Option<Self> {
        self.raw.checked_add(rhs.raw).map(StrIndex::from)
    }

    /// Checked integer subtraction.
    pub fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.raw.checked_sub(rhs.raw).map(StrIndex::from)
    }
}

/// A range of a string, represented as a half-open range of `StrIndex`.
///
/// Construct a `StrRange` by using `from` conversion from `std::ops::Range`/`RangeTo`.
/// The range is always guaranteed increasing; conversion panics if `end < start`.
///
/// # Examples
///
/// ```rust
/// # use str_index::{StrRange, StrIndex};
/// let zero = StrIndex::from(0);
/// let start = StrIndex::from(10);
/// let end = StrIndex::from(20);
/// assert_eq!(
///     format!("{:?}", StrRange::from(start..end)),
///     format!("{:?}", start..end),
/// );
/// assert_eq!(
///     format!("{:?}", StrRange::from(..end)),
///     format!("{:?}", zero..end),
/// );
/// ```
///
/// ```rust,should_panic
/// # use str_index::{StrRange, StrIndex};
/// # let start = StrIndex::from(10);
/// # let end = StrIndex::from(20);
/// let this_panics = StrRange::from(end..start);
/// ```
#[derive(Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct StrRange {
    start: StrIndex,
    end: StrIndex,
}

impl StrRange {
    /// The (inclusive) start index of this range.
    pub fn start(self) -> StrIndex {
        self.start
    }

    /// The (exclusive) end index of this range.
    pub fn end(self) -> StrIndex {
        self.end
    }
}
