use {
    crate::{StrIndex, StrRange},
    core::{
        convert::{TryFrom, TryInto},
        ops::{Range, RangeTo},
    },
};

impl From<StrIndex> for u32 {
    fn from(idx: StrIndex) -> Self {
        idx.raw
    }
}

impl From<u32> for StrIndex {
    fn from(i: u32) -> Self {
        StrIndex { raw: i }
    }
}

impl From<StrIndex> for usize {
    fn from(idx: StrIndex) -> Self {
        idx.raw as usize
    }
}

impl TryFrom<usize> for StrIndex {
    type Error = <usize as TryInto<u32>>::Error;
    fn try_from(i: usize) -> Result<Self, Self::Error> {
        i.try_into().map(|raw| StrIndex { raw })
    }
}

impl From<Range<StrIndex>> for StrRange {
    fn from(range: Range<StrIndex>) -> Self {
        let Range { start, end } = range;
        let range = StrRange { start, end };
        assert!(start <= end, "invalid string range {}", range);
        range
    }
}

impl From<RangeTo<StrIndex>> for StrRange {
    fn from(range: RangeTo<StrIndex>) -> Self {
        StrRange {
            start: 0.into(),
            end: range.end,
        }
    }
}
