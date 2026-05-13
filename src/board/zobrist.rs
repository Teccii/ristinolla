use crate::types::{Piece, Square};

#[derive(Debug, Clone)]
pub struct SplitMix64 {
    pub state: u64,
}

impl SplitMix64 {
    #[inline]
    pub const fn new(state: u64) -> SplitMix64 {
        SplitMix64 { state }
    }

    #[inline]
    pub const fn next(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9e3779b97f4a7c15u64);

        let mut temp = self.state;
        temp = (temp ^ (temp >> 30)).wrapping_mul(0xbf58476d1ce4e5b9u64);
        temp = (temp ^ (temp >> 27)).wrapping_mul(0x94d049bb133111ebu64);

        temp ^ (temp >> 31)
    }
}

/*----------------------------------------------------------------*/

#[derive(Debug, Clone)]
pub struct Zobrist {
    pub pieces: [[u64; Square::COUNT]; Piece::COUNT],
    pub stm: u64,
}

impl Zobrist {
    #[inline]
    pub const fn new(seed: u64) -> Zobrist {
        let mut rng = SplitMix64::new(seed);
        let mut zobrist = Zobrist {
            pieces: [[0; Square::COUNT]; Piece::COUNT],
            stm: 0,
        };

        let mut piece = 0;
        while piece < Piece::COUNT {
            let mut sq = 0;
            while sq < Square::COUNT {
                zobrist.pieces[piece][sq] = rng.next();
                sq += 1;
            }
            piece += 1;
        }

        zobrist.stm = rng.next();
        zobrist
    }
}

pub static ZOBRIST: Zobrist = Zobrist::new(0xe65f2056120a3513);
