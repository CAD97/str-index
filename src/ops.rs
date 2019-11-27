use {
    crate::{StrIndex, StrRange},
    core::ops::{Add, AddAssign, Bound, Index, IndexMut, RangeBounds, Sub, SubAssign},
};

macro_rules! math {
    (impl $T:ident for $t:ident by fn $f:ident = $o:tt) => {
        impl $T<$t> for $t {
            type Output = $t;
            fn $f(self, rhs: $t) -> $t {
                $t::from(self.raw $o rhs.raw)
            }
        }
        impl $T<$t> for &$t {
            type Output = $t;
            fn $f(self, rhs: $t) -> $t {
                $t::from(self.raw $o rhs.raw)
            }
        }
        impl $T<&$t> for $t {
            type Output = $t;
            fn $f(self, rhs: &$t) -> $t {
                $t::from(self.raw $o rhs.raw)
            }
        }
        impl $T<&$t> for &$t {
            type Output = $t;
            fn $f(self, rhs: &$t) -> $t {
                $t::from(self.raw $o rhs.raw)
            }
        }
    }
}

math!(impl Add for StrIndex by fn add = +);
math!(impl Sub for StrIndex by fn sub = -);

impl<Rhs> AddAssign<Rhs> for StrIndex
where
    Self: Add<Rhs, Output = Self>,
{
    fn add_assign(&mut self, rhs: Rhs) {
        *self = *self + rhs;
    }
}

impl<Rhs> SubAssign<Rhs> for StrIndex
where
    Self: Sub<Rhs, Output = Self>,
{
    fn sub_assign(&mut self, rhs: Rhs) {
        *self = *self - rhs;
    }
}

impl Index<StrRange> for str {
    type Output = str;

    fn index(&self, index: StrRange) -> &str {
        &self[index.start().into()..index.end().into()]
    }
}

impl IndexMut<StrRange> for str {
    fn index_mut(&mut self, index: StrRange) -> &mut Self::Output {
        &mut self[index.start().into()..index.end().into()]
    }
}

impl RangeBounds<StrIndex> for StrRange {
    fn start_bound(&self) -> Bound<&StrIndex> {
        Bound::Included(&self.start)
    }

    fn end_bound(&self) -> Bound<&StrIndex> {
        Bound::Excluded(&self.end)
    }
}

#[cfg(feature = "alloc")]
mod for_alloc_types {
    use {super::*, alloc::string::String};

    impl Index<StrRange> for String {
        type Output = str;
        fn index(&self, index: StrRange) -> &str {
            &self[..][index]
        }
    }

    impl IndexMut<StrRange> for String {
        fn index_mut(&mut self, index: StrRange) -> &mut Self::Output {
            &mut self[..][index]
        }
    }

    #[test]
    fn string_indexing() {
        let range = StrRange::from(0.into()..5.into());
        let s = String::from("swordfish");
        assert_eq!(&s[range], "sword");
    }
}

#[test]
#[allow(clippy::no_effect)]
fn str_indexing() {
    let range = StrRange::from(0.into()..5.into());
    let s = "swordfish";
    assert_eq!(&s[range], "sword");
}
