use crate::def_enum;
use enum_map::Enum;
use std::{fmt, str::FromStr};

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

impl fmt::Display for Rank {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct RankParseError;

impl From<Rank> for char {
    #[inline]
    fn from(r: Rank) -> char {
        match r {
            Rank::First => '1',
            Rank::Second => '2',
            Rank::Third => '3',
            Rank::Fourth => '4',
            Rank::Fifth => '5',
            Rank::Sixth => '6',
            Rank::Seventh => '7',
            Rank::Eighth => '8',
            Rank::Ninth => '9',
        }
    }
}

impl TryFrom<char> for Rank {
    type Error = RankParseError;

    #[inline]
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c.to_ascii_lowercase() {
            '1' => Ok(Rank::First),
            '2' => Ok(Rank::Second),
            '3' => Ok(Rank::Third),
            '4' => Ok(Rank::Fourth),
            '5' => Ok(Rank::Fifth),
            '6' => Ok(Rank::Sixth),
            '7' => Ok(Rank::Seventh),
            '8' => Ok(Rank::Eighth),
            '9' => Ok(Rank::Ninth),
            _ => Err(RankParseError),
        }
    }
}

impl FromStr for Rank {
    type Err = RankParseError;

    #[inline]
    fn from_str(s: &str) -> Result<Rank, RankParseError> {
        let mut chars = s.chars();
        let c = chars.next().ok_or(RankParseError)?;

        if chars.next().is_none() {
            c.try_into()
        } else {
            Err(RankParseError)
        }
    }
}
