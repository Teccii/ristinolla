use crate::def_enum;
use enum_map::Enum;

def_enum! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Enum)]
    pub enum Rank : u8 {
        First,
        Second,
        Third,
        Fourth,
        Fifth,
        Sixth,
        Seventh,
        Eighth,
        Ninth
    }
}

impl Rank {
    #[inline]
    pub const fn offset(self, dy: i8) -> Self {
        let i = self as i8 + dy;
        Self::index(i as usize)
    }

    #[inline]
    pub const fn try_offset(self, dy: i8) -> Option<Self> {
        let i = self as i8 + dy;

        if i < 0 || i >= Self::COUNT as i8 {
            return None;
        }

        Self::try_index(i as usize)
    }

    #[inline]
    pub const fn flip(self) -> Self {
        Self::index(Rank::Ninth as usize - self as usize)
    }
}
