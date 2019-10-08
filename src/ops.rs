use {
    crate::StrIndex,
    core::ops::{Add, AddAssign, Sub, SubAssign},
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
