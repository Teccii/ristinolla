use crate::{
    def_enum,
    types::{Bitboard, File, Rank},
};
use enum_map::Enum;
use std::{fmt, str::FromStr};

def_enum! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Enum)]
    pub enum Square : u8 {
        A1, B1, C1, D1, E1, F1, G1, H1, I1,
        A2, B2, C2, D2, E2, F2, G2, H2, I2,
        A3, B3, C3, D3, E3, F3, G3, H3, I3,
        A4, B4, C4, D4, E4, F4, G4, H4, I4,
        A5, B5, C5, D5, E5, F5, G5, H5, I5,
        A6, B6, C6, D6, E6, F6, G6, H6, I6,
        A7, B7, C7, D7, E7, F7, G7, H7, I7,
        A8, B8, C8, D8, E8, F8, G8, H8, I8,
        A9, B9, C9, D9, E9, F9, G9, H9, I9
    }
}

impl Square {
    #[inline]
    pub const fn new(file: File, rank: Rank) -> Square {
        Square::index(rank as usize * 9 + file as usize)
    }

    /*----------------------------------------------------------------*/

    #[inline]
    pub const fn flip_file(self) -> Square {
        Square::new(self.file().flip(), self.rank())
    }

    #[inline]
    pub const fn flip_rank(self) -> Square {
        Square::new(self.file(), self.rank().flip())
    }

    #[inline]
    pub const fn offset(self, dx: i8, dy: i8) -> Square {
        let file = self.file().offset(dx);
        let rank = self.rank().offset(dy);

        Square::new(file, rank)
    }

    #[inline]
    pub fn try_offset(self, dx: i8, dy: i8) -> Option<Square> {
        let file = self.file().try_offset(dx)?;
        let rank = self.rank().try_offset(dy)?;

        Some(Square::new(file, rank))
    }

    /*----------------------------------------------------------------*/

    #[inline]
    pub const fn indices(self) -> (usize, usize) {
        use Square::*;

        match self {
            A1 => (0, 0),
            A2 => (0, 3),
            A3 => (0, 6),
            B1 => (0, 1),
            B2 => (0, 4),
            B3 => (0, 7),
            C1 => (0, 2),
            C2 => (0, 5),
            C3 => (0, 8),
            D1 => (1, 0),
            D2 => (1, 3),
            D3 => (1, 6),
            E1 => (1, 1),
            E2 => (1, 4),
            E3 => (1, 7),
            F1 => (1, 2),
            F2 => (1, 5),
            F3 => (1, 8),
            G1 => (2, 0),
            G2 => (2, 3),
            G3 => (2, 6),
            H1 => (2, 1),
            H2 => (2, 4),
            H3 => (2, 7),
            I1 => (2, 2),
            I2 => (2, 5),
            I3 => (2, 8),

            A4 => (3, 0),
            A5 => (3, 3),
            A6 => (3, 6),
            B4 => (3, 1),
            B5 => (3, 4),
            B6 => (3, 7),
            C4 => (3, 2),
            C5 => (3, 5),
            C6 => (3, 8),
            D4 => (4, 0),
            D5 => (4, 3),
            D6 => (4, 6),
            E4 => (4, 1),
            E5 => (4, 4),
            E6 => (4, 7),
            F4 => (4, 2),
            F5 => (4, 5),
            F6 => (4, 8),
            G4 => (5, 0),
            G5 => (5, 3),
            G6 => (5, 6),
            H4 => (5, 1),
            H5 => (5, 4),
            H6 => (5, 7),
            I4 => (5, 2),
            I5 => (5, 5),
            I6 => (5, 8),

            A7 => (6, 0),
            A8 => (6, 3),
            A9 => (6, 6),
            B7 => (6, 1),
            B8 => (6, 4),
            B9 => (6, 7),
            C7 => (6, 2),
            C8 => (6, 5),
            C9 => (6, 8),
            D7 => (7, 0),
            D8 => (7, 3),
            D9 => (7, 6),
            E7 => (7, 1),
            E8 => (7, 4),
            E9 => (7, 7),
            F7 => (7, 2),
            F8 => (7, 5),
            F9 => (7, 8),
            G7 => (8, 0),
            G8 => (8, 3),
            G9 => (8, 6),
            H7 => (8, 1),
            H8 => (8, 4),
            H9 => (8, 7),
            I7 => (8, 2),
            I8 => (8, 5),
            I9 => (8, 8),
        }
    }

    #[inline]
    pub const fn file(self) -> File {
        File::index(self as usize % 9)
    }

    #[inline]
    pub const fn rank(self) -> Rank {
        Rank::index(self as usize / 9)
    }

    #[inline]
    pub const fn bitboard(self) -> Bitboard {
        Bitboard::new(1u128 << self as u8)
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.file(), self.rank())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SquareParseError {
    InvalidFile,
    InvalidRank,
}

impl FromStr for Square {
    type Err = SquareParseError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let file = chars
            .next()
            .and_then(|c| File::try_from(c).ok())
            .ok_or(SquareParseError::InvalidFile)?;
        let rank = chars
            .next()
            .and_then(|c| Rank::try_from(c).ok())
            .ok_or(SquareParseError::InvalidRank)?;

        Ok(Square::new(file, rank))
    }
}
