use crate::types::Square;
use std::ops::*;

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Bitboard(u128);

impl Bitboard {
    #[inline]
    pub const fn new(x: u128) -> Self {
        Bitboard(x & Bitboard::FULL.0)
    }

    /*----------------------------------------------------------------*/

    #[inline]
    pub const fn next_square(self) -> Square {
        Square::index(self.0.trailing_zeros() as usize)
    }

    #[inline]
    pub const fn try_next_square(self) -> Option<Square> {
        Square::try_index(self.0.trailing_zeros() as usize)
    }

    /*----------------------------------------------------------------*/

    #[inline]
    pub const fn is_superset(self, rhs: Bitboard) -> bool {
        rhs.is_subset(self)
    }

    #[inline]
    pub const fn is_subset(self, rhs: Bitboard) -> bool {
        self.0 & rhs.0 == self.0
    }

    #[inline]
    pub const fn is_disjoint(self, rhs: Bitboard) -> bool {
        self.0 & rhs.0 == 0
    }

    /*----------------------------------------------------------------*/

    #[inline]
    pub const fn raw(self) -> u128 {
        self.0
    }

    #[inline]
    pub const fn popcnt(self) -> usize {
        self.0.count_ones() as usize
    }

    #[inline]
    pub const fn has(self, sq: Square) -> bool {
        !self.is_disjoint(sq.bitboard())
    }

    #[inline]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    /*----------------------------------------------------------------*/

    pub const EMPTY: Bitboard = Bitboard(0);
    pub const FULL: Bitboard = Bitboard(u128::MAX >> (128 - 81));
}

impl Not for Bitboard {
    type Output = Self;

    #[inline]
    fn not(self) -> Self::Output {
        Bitboard(!self.0) & Bitboard::FULL
    }
}

macro_rules! impl_bb_ops {
    ($($trait:ident, $fn:ident;)*) => {$(
        impl $trait for Bitboard {
            type Output = Self;

            #[inline]
            fn $fn(self, rhs: Self) -> Self::Output {
                Bitboard(self.0.$fn(rhs.0))
            }
        }
    )*}
}

macro_rules! impl_bb_assign_ops {
    ($($trait:ident, $fn:ident;)*) => {$(
        impl $trait for Bitboard {
            #[inline]
            fn $fn(&mut self, rhs: Self) {
                self.0.$fn(rhs.0);
            }
        }
    )*}
}

macro_rules! impl_bb_shift_ops {
    ($($ty:ty,)*) => {$(
        impl Shl<$ty> for Bitboard {
            type Output = Self;

            #[inline]
            fn shl(self, rhs: $ty) -> Self::Output {
                Bitboard(self.0 << rhs) & Bitboard::FULL
            }
        }

        impl Shr<$ty> for Bitboard {
            type Output = Self;

            #[inline]
            fn shr(self, rhs: $ty) -> Self::Output {
                Bitboard(self.0 >> rhs)
            }
        }

        impl ShlAssign<$ty> for Bitboard {
            #[inline]
            fn shl_assign(&mut self, rhs: $ty) {
                self.0 <<= rhs;
                *self &= Bitboard::FULL;
            }
        }

        impl ShrAssign<$ty> for Bitboard {
            #[inline]
            fn shr_assign(&mut self, rhs: $ty) {
                self.0 >>= rhs;
            }
        }
    )*}
}

impl_bb_ops! {
    BitAnd, bitand;
    BitOr, bitor;
    BitXor, bitxor;
}

impl_bb_assign_ops! {
    BitAndAssign, bitand_assign;
    BitOrAssign, bitor_assign;
    BitXorAssign, bitxor_assign;
}

impl_bb_shift_ops! {
    u8, u16, u32, u64, usize,
}

/*----------------------------------------------------------------*/

#[derive(Debug, Clone)]
pub struct BitboardIter(Bitboard);

impl IntoIterator for Bitboard {
    type Item = Square;
    type IntoIter = BitboardIter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        BitboardIter(self)
    }
}

impl Iterator for BitboardIter {
    type Item = Square;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let sq = self.0.try_next_square();
        if let Some(sq) = sq {
            self.0 ^= sq.bitboard();
        }

        sq
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl ExactSizeIterator for BitboardIter {
    #[inline]
    fn len(&self) -> usize {
        self.0.popcnt()
    }
}
