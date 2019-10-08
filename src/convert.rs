use {
    crate::StrIndex,
    core::convert::{TryFrom, TryInto},
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
