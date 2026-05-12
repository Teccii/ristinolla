use crate::{
    def_enum,
    types::{Bitboard, File, Rank},
};
use enum_map::Enum;

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
