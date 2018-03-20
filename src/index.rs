use std::{iter::FromIterator, ops::{BitOr, BitOrAssign}};

bitflags! {
    pub struct Index: u8 {
        const SUBJECT   = 0b0001;
        const PREDICATE = 0b0010;
        const OBJECT    = 0b0100;
        const CONTEXT   = 0b1000;
    }
}

/// A bitset where each set bit represents an `Index`.
///
/// This type is a bit tricky; it is internally represented as a `u16`, where the lowest bit is
/// *never* set.
#[derive(Debug, Default, Clone, Copy)]
pub struct Indices {
    bits: u16,
}

impl Indices {
    pub const ALL: Indices = Self { bits: !1 };

    pub fn contains(&self, index: Index) -> bool {
        self.bits & (1 << index.bits()) != 0
    }
}

impl FromIterator<Index> for Indices {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Index>,
    {
        let mut this = Self::default();

        for index in iter {
            this.bits |= 1 << index.bits();
        }

        this
    }
}

impl IntoIterator for Indices {
    type Item = Index;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self.bits)
    }
}

impl BitOrAssign<Index> for Indices {
    fn bitor_assign(&mut self, rhs: Index) {
        self.bits |= rhs.bits() as u16;
    }
}

impl BitOr for Indices {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Indices {
            bits: self.bits | rhs.bits,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct IntoIter(u16);

impl Iterator for IntoIter {
    type Item = Index;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 != 0 {
            let lz = self.0.trailing_zeros();
            let idx = Index::from_bits_truncate(lz as u8);
            self.0 ^= 1 << lz;
            Some(idx)
        } else {
            None
        }
    }
}
