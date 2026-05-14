use crate::def_enum;
use enum_map::Enum;
use std::{fmt, str::FromStr};

def_enum! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Enum)]
    pub enum File : u8 {
        A,
        B,
        C,
        D,
        E,
        F,
        G,
        H,
        I
    }
}

impl File {
    #[inline]
    pub const fn offset(self, dx: i8) -> Self {
        let i = self as i8 + dx;
        Self::index(i as usize)
    }

    #[inline]
    pub const fn try_offset(self, dx: i8) -> Option<Self> {
        let i = self as i8 + dx;

        if i < 0 || i >= Self::COUNT as i8 {
            return None;
        }

        Self::try_index(i as usize)
    }

    #[inline]
    pub const fn flip(self) -> Self {
        Self::index(File::I as usize - self as usize)
    }
}

impl fmt::Display for File {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct FileParseError;

impl From<File> for char {
    #[inline]
    fn from(f: File) -> char {
        match f {
            File::A => 'a',
            File::B => 'b',
            File::C => 'c',
            File::D => 'd',
            File::E => 'e',
            File::F => 'f',
            File::G => 'g',
            File::H => 'h',
            File::I => 'i',
        }
    }
}

impl TryFrom<char> for File {
    type Error = FileParseError;

    #[inline]
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c.to_ascii_lowercase() {
            'a' => Ok(File::A),
            'b' => Ok(File::B),
            'c' => Ok(File::C),
            'd' => Ok(File::D),
            'e' => Ok(File::E),
            'f' => Ok(File::F),
            'g' => Ok(File::G),
            'h' => Ok(File::H),
            'i' => Ok(File::I),
            _ => Err(FileParseError),
        }
    }
}

impl FromStr for File {
    type Err = FileParseError;

    #[inline]
    fn from_str(s: &str) -> Result<File, FileParseError> {
        let mut chars = s.chars();
        let c = chars.next().ok_or(FileParseError)?;

        if chars.next().is_none() {
            c.try_into()
        } else {
            Err(FileParseError)
        }
    }
}
