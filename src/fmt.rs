use {
    crate::{StrIndex, StrRange},
    core::fmt,
};

impl fmt::Debug for StrIndex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.raw, f)
    }
}

impl fmt::Display for StrIndex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.raw, f)
    }
}

impl fmt::Debug for StrRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&(self.start()..self.end()), f)
    }
}

impl fmt::Display for StrRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}..{}", self.start(), self.end())
    }
}
