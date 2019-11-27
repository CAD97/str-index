#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

use core::{cmp, u32};

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

    /// A range starting at this index.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use str_index::*;
    /// let point = StrIndex::from(5);
    /// assert_eq!(
    ///     point.range_for(10.into()),
    ///     StrRange::from(5.into()..15.into()),
    /// );
    /// ```
    pub fn range_for(self, len: StrIndex) -> StrRange {
        StrRange::from(self..self + len)
    }

    /// A range from this index to another.
    ///
    /// # Panics
    ///
    /// Panics if `end < self`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use str_index::*;
    /// let start = StrIndex::from(0);
    /// let end = StrIndex::from(10);
    /// assert_eq!(
    ///     start.range_to(end),
    ///     StrRange::from(start..end),
    /// );
    /// ```
    pub fn range_to(self, end: StrIndex) -> StrRange {
        StrRange::from(self..end)
    }

    /// The empty range at this index.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use str_index::*;
    /// let point = StrIndex::from(10);
    /// assert_eq!(
    ///     point.as_unit_range(),
    ///     point.range_to(point),
    /// );
    /// ```
    pub fn as_unit_range(self) -> StrRange {
        StrRange::from(self..self)
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

    /// The length of this range.
    pub fn len(self) -> StrIndex {
        self.end() - self.start()
    }

    /// Is this range a unit range?
    /// That is, does this range have equivalent start and end points?
    pub fn is_empty(self) -> bool {
        self.start() == self.end()
    }

    /// A range with an adjusted end.
    ///
    /// # Panics
    ///
    /// Panics if `self.end() < start`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use str_index::*;
    /// let range = StrRange::from(5.into()..10.into());
    /// let point = StrIndex::from(0);
    /// assert_eq!(
    ///     range.with_start(point),
    ///     point.range_to(range.end()),
    /// );
    /// ```
    pub fn with_start(self, start: StrIndex) -> StrRange {
        StrRange::from(start..self.end())
    }

    /// A range with an adjusted end.
    ///
    /// # Panics
    ///
    /// Panics if `end < self.start()`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use str_index::*;
    /// let range = StrRange::from(0.into()..5.into());
    /// let point = StrIndex::from(10);
    /// assert_eq!(
    ///     range.with_end(point),
    ///     range.start().range_to(point),
    /// );
    /// ```
    pub fn with_end(self, end: StrIndex) -> StrRange {
        StrRange::from(self.start()..end)
    }

    /// Are these ranges disjoint?
    ///
    /// Ranges that touch end to start are disjoint, as no byte is in both ranges.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use str_index::*;
    /// let left = StrRange::from(0.into()..20.into());
    /// let right = StrRange::from(10.into()..30.into());
    /// assert!(!left.is_disjoint(right));
    /// assert!(!right.is_disjoint(left));
    ///
    /// let left = StrRange::from(0.into()..10.into());
    /// let right = StrRange::from(10.into()..20.into());
    /// assert!(left.is_disjoint(right));
    /// assert!(right.is_disjoint(left));
    ///
    /// let empty = StrRange::from(10.into()..10.into());
    /// assert!(empty.is_disjoint(empty));
    /// ```
    pub fn is_disjoint(self, other: StrRange) -> bool {
        self.end() <= other.start() || other.end() <= self.start()
    }

    /// Does this range contain `other`?
    ///
    /// `other` must be completely within `self`, but may share endpoints.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use str_index::*;
    /// let range = StrRange::from(0.into()..20.into());
    /// assert!(range.contains(StrRange::from(5.into()..15.into())));
    /// assert!(range.contains(range));
    /// ```
    pub fn contains(self, other: StrRange) -> bool {
        self.start() <= other.start() && other.end() <= self.end()
    }

    /// Does this range contain this index?
    ///
    /// This is an exclusive test; use `StrIndex::as_unit_range` for an inclusive test.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use str_index::*;
    /// let range = StrRange::from(10.into()..20.into());
    /// assert!(range.contains_exclusive(range.start()));
    /// assert!(!range.contains_exclusive(range.end()));
    /// assert!(range.contains(range.end().as_unit_range()));
    /// ```
    pub fn contains_exclusive(self, index: StrIndex) -> bool {
        self.start() <= index && index < self.end()
    }

    /// The range that is both in `self` and `other`.
    ///
    /// Note that ranges that touch but do not overlap return `Some(empty range)`
    /// and ranges that do not touch and do not overlap return `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use str_index::*;
    /// let left = StrRange::from(0.into()..20.into());
    /// let right = StrRange::from(10.into()..30.into());
    /// assert_eq!(
    ///     left.intersection(right),
    ///     Some(StrRange::from(10.into()..20.into())),
    /// );
    ///
    /// let left = StrRange::from(0.into()..10.into());
    /// let right = StrRange::from(10.into()..20.into());
    /// assert_eq!(
    ///     left.intersection(right),
    ///     Some(StrRange::from(10.into()..10.into())),
    /// );
    /// ```
    pub fn intersection(self, other: Self) -> Option<Self> {
        let start = cmp::max(self.start(), other.start());
        let end = cmp::min(self.end(), other.end());
        if start <= end {
            Some(StrRange::from(start..end))
        } else {
            None
        }
    }

    /// Like [`intersection`], but disjoint ranges always return `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use str_index::*;
    /// let left = StrRange::from(0.into()..20.into());
    /// let right = StrRange::from(10.into()..30.into());
    /// assert_eq!(
    ///     left.nonempty_intersection(right),
    ///     left.intersection(right),
    /// );
    ///
    /// let left = StrRange::from(0.into()..10.into());
    /// let right = StrRange::from(10.into()..20.into());
    /// assert_eq!(
    ///     left.nonempty_intersection(right),
    ///     None,
    /// );
    /// ```
    pub fn nonempty_intersection(self, other: Self) -> Option<Self> {
        let start = cmp::max(self.start(), other.start());
        let end = cmp::min(self.end(), other.end());
        if start < end {
            Some(StrRange::from(start..end))
        } else {
            None
        }
    }

    /// The range that covers both `self` and `other`.
    ///
    /// Note that it will also cover any space between `self` and `other`.
    ///
    /// # Example
    ///
    /// ```
    /// # use str_index::*;
    /// let left = StrRange::from(0.into()..10.into());
    /// let right = StrRange::from(20.into()..30.into());
    /// assert_eq!(
    ///     left.merge(right),
    ///     StrRange::from(0.into()..30.into()),
    /// );
    /// ```
    pub fn merge(self, other: Self) -> Self {
        let start = cmp::min(self.start(), other.start());
        let end = cmp::max(self.end(), other.end());
        StrRange::from(start..end)
    }
}
