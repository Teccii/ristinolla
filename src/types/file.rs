use crate::def_enum;
use enum_map::Enum;

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
