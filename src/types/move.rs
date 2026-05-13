use crate::types::Square;
use std::num::NonZeroU8;

/*
Bit Layout
- Bit 0: Always true for niche value optimisation
- Bits 1-7: Destination square
*/
#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Move(NonZeroU8);

impl Move {
    #[inline]
    pub const fn new(sq: Square) -> Move {
        Move(NonZeroU8::new(1 | ((sq as u8) << 1)).unwrap())
    }

    #[inline]
    pub const fn dest(self) -> Square {
        Square::index((self.0.get() >> 1) as usize)
    }
}
