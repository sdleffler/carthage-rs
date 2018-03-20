use std::{iter::FromIterator, ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Sub, SubAssign}};

use bitwise::word::ParallelBitsDeposit;

bitflags! {
    /// A bitset representing a single index constructed for an RDF quadstore.
    ///
    /// Setting a flag in the index set will add that key to the index; so an `Index` constructed
    /// as `Index::SUBJECT | Index::PREDICATE | Index::CONTEXT` will allow for searches over
    /// concrete subjects, predicates, and contexts together, returning all quads the
    /// subjects/predicates/contexts of which match the search query, and the objects of which
    /// will vary.
    pub struct Index: u8 {
        const SUBJECT   = 0b1000;
        const PREDICATE = 0b0100;
        const OBJECT    = 0b0010;
        const CONTEXT   = 0b0001;
    }
}

/// A set of `Index`es for initializing a given quadstore. This is implemented as a bitset where
/// each set bit represents an `Index`.
#[derive(Debug, Default, Clone, Copy)]
pub struct IndexSet {
    bits: u16,
}

impl IndexSet {
    /// All available indices.
    pub const ALL: IndexSet = Self { bits: !1 }; // NB `!1` because the lowest bit is always unset.

    /// Empty index set.
    pub const EMPTY: IndexSet = Self { bits: 0 };

    /// Check to see if this set of indices supports the given index.
    pub fn contains(&self, index: Index) -> bool {
        self.bits & (1 << index.bits()) != 0
    }

    /// Enumerate through indices which form workable approximations of a given index; in other
    /// words, for a given index, take the intersection of its power set with the index set and
    /// then pick one of the elements with the most set bits.
    pub fn find_approximation(&self, index: Index) -> Option<Index> {
        // NB This is a bit-tricky. Because it's a bit trick. Get it?
        //
        // We take the power set of the index we're trying to approximate, and then intersect it
        // with all indices in our index set. We use PDEP (parallel bits deposit) to enumerate
        // through the power set.
        //
        // This will be v. fast on systems which support BMI2, because PDEP compiles down to a
        // single instruction. Not sure it needs to be that fast, honestly, but hey, it's cool
        // right?
        let index_set = index.bits() as u8;
        let mut power_set = (0..(1 << index_set.count_ones()))
            .rev()
            .map(|x| Index::from_bits_truncate(x.parallel_bits_deposit(index_set)))
            .filter(|&x| self.contains(x));

        power_set.next()
    }
}

impl Extend<Index> for IndexSet {
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = Index>,
    {
        iter.into_iter().for_each(|idx| *self |= idx);
    }
}

impl FromIterator<Index> for IndexSet {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Index>,
    {
        let mut this = Self::default();
        this.extend(iter);
        this
    }
}

impl<'a> FromIterator<&'a Index> for IndexSet {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = &'a Index>,
    {
        iter.into_iter().map(|&x| x).collect()
    }
}

impl IntoIterator for IndexSet {
    type Item = Index;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self.bits)
    }
}

impl BitOrAssign<Index> for IndexSet {
    fn bitor_assign(&mut self, rhs: Index) {
        self.bits |= 1 << rhs.bits();
    }
}

impl BitOr for IndexSet {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        IndexSet {
            bits: self.bits | rhs.bits,
        }
    }
}

impl BitOrAssign for IndexSet {
    fn bitor_assign(&mut self, rhs: Self) {
        self.bits |= rhs.bits;
    }
}

impl BitAnd for IndexSet {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        IndexSet {
            bits: self.bits & rhs.bits,
        }
    }
}

impl BitAndAssign for IndexSet {
    fn bitand_assign(&mut self, rhs: Self) {
        self.bits &= rhs.bits;
    }
}

impl Sub for IndexSet {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        IndexSet {
            bits: self.bits & !rhs.bits,
        }
    }
}

impl SubAssign for IndexSet {
    fn sub_assign(&mut self, rhs: Self) {
        self.bits &= !rhs.bits;
    }
}

/// Iterator over all indices stored in an index set.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn approximate() {
        let set: IndexSet = [
            Index::SUBJECT | Index::PREDICATE | Index::CONTEXT,
            Index::SUBJECT | Index::PREDICATE | Index::OBJECT,
            Index::SUBJECT | Index::PREDICATE,
            Index::SUBJECT,
            Index::PREDICATE | Index::CONTEXT,
            Index::PREDICATE,
            Index::OBJECT,
        ].iter()
            .collect();

        assert_eq!(
            set.find_approximation(Index::PREDICATE | Index::OBJECT),
            Some(Index::PREDICATE)
        );
        assert_eq!(set.find_approximation(Index::CONTEXT), None);
        assert_eq!(
            set.find_approximation(Index::SUBJECT | Index::PREDICATE),
            Some(Index::SUBJECT | Index::PREDICATE)
        );
        assert_eq!(set.find_approximation(Index::empty()), None);
        assert_eq!(
            set.find_approximation(Index::all()),
            Some(Index::SUBJECT | Index::PREDICATE | Index::OBJECT)
        );
    }
}
