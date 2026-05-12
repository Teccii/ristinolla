use crate::def_enum;
use enum_map::Enum;
use std::ops::Not;

def_enum! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Enum)]
    pub enum Color : u8 {
        White,
        Black
    }
}

impl Not for Color {
    type Output = Self;

    #[inline]
    fn not(self) -> Self::Output {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}
