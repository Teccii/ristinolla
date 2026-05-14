use crate::def_enum;
use enum_map::Enum;
use std::{fmt, ops::Not, str::FromStr};

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

impl fmt::Display for Piece {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct PieceParseError;

impl From<Piece> for char {
    #[inline]
    fn from(p: Piece) -> char {
        match p {
            Piece::X => 'x',
            Piece::O => 'o',
        }
    }
}

impl TryFrom<char> for Piece {
    type Error = PieceParseError;

    #[inline]
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c.to_ascii_lowercase() {
            'x' => Ok(Piece::X),
            'o' => Ok(Piece::O),
            _ => Err(PieceParseError),
        }
    }
}

impl FromStr for Piece {
    type Err = PieceParseError;

    #[inline]
    fn from_str(s: &str) -> Result<Piece, PieceParseError> {
        let mut chars = s.chars();
        let c = chars.next().ok_or(PieceParseError)?;

        if chars.next().is_none() {
            c.try_into()
        } else {
            Err(PieceParseError)
        }
    }
}
