use crate::def_enum;
use enum_map::Enum;
use std::ops::Not;

def_enum! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Enum, Default)]
    pub enum Piece : u8 {
        #[default] X,
        O
    }
}

impl Not for Piece {
    type Output = Self;

    #[inline]
    fn not(self) -> Self::Output {
        match self {
            Self::X => Self::O,
            Self::O => Self::X,
        }
    }
}
