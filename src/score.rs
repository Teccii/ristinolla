use crate::search::MAX_PLY;
use std::{fmt, ops::*};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Score(pub i32);

impl Score {
    #[inline]
    pub fn mate(ply: u8) -> Score {
        Score::MIN_MATE - ply as i32
    }

    #[inline]
    pub fn mated(ply: u8) -> Score {
        -Score::MIN_MATE + ply as i32
    }

    /*----------------------------------------------------------------*/

    #[inline]
    pub fn is_inf(self) -> bool {
        let abs_score = self.abs();
        abs_score > Score::MIN_MATE
    }

    #[inline]
    pub fn is_mate(self) -> bool {
        let abs_score = self.abs();
        abs_score >= Score::MAX_MATE && abs_score <= Score::MIN_MATE
    }

    #[inline]
    pub fn is_win(self) -> bool {
        self >= Score::MAX_MATE
    }

    #[inline]
    pub fn is_loss(self) -> bool {
        self <= -Score::MAX_MATE
    }

    #[inline]
    pub fn mate_in(self) -> Option<i16> {
        if self.is_mate() {
            let abs_score = self.abs();
            let sign = self.sign() as i16;
            let ply = sign * (Score::MIN_MATE.0 - abs_score.0) as i16;

            return Some(ply);
        }

        None
    }

    /*----------------------------------------------------------------*/

    #[inline]
    pub const fn abs(self) -> Score {
        Score(self.0.abs())
    }

    #[inline]
    pub const fn sign(self) -> i32 {
        self.0.signum()
    }

    /*----------------------------------------------------------------*/

    pub const MIN_MATE: Score = Score(i16::MAX as i32 - MAX_PLY as i32);
    pub const MAX_MATE: Score = Score(i16::MAX as i32 - (2 * MAX_PLY) as i32);
    pub const INF: Score = Score(i16::MAX as i32);
    pub const NONE: Score = Score(i16::MIN as i32);
}

impl fmt::Display for Score {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if f.alternate() {
            if self.is_inf() {
                if self.0 > 0 {
                    write!(f, "+INF")
                } else {
                    write!(f, "-INF")
                }
            } else if let Some(ply) = self.mate_in() {
                write!(f, "#{}", ply)
            } else {
                write!(f, "{:+.2}", self.0 as f32 / 100.0)
            }
        } else {
            if let Some(ply) = self.mate_in() {
                write!(f, "mate {}", ply)
            } else {
                write!(f, "cp {}", self.0)
            }
        }
    }
}

impl Neg for Score {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Score(-self.0)
    }
}

macro_rules! impl_score_ops {
    ($($trait:ident, $fn:ident;)*) => {$(
        impl $trait for Score {
            type Output = Self;

            #[inline]
            fn $fn(self, rhs: Score) -> Self::Output {
                Score(self.0.$fn(rhs.0))
            }
        }

        impl $trait<i32> for Score {
            type Output = Self;

            #[inline]
            fn $fn(self, rhs: i32) -> Self::Output {
                Score(self.0.$fn(rhs))
            }
        }

        impl $trait<Score> for i32 {
            type Output = Score;

            #[inline]
            fn $fn(self, rhs: Score) -> Self::Output {
                Score(self.$fn(rhs.0))
            }
        }
    )*};
}

macro_rules! impl_score_assign_ops {
    ($($trait:ident, $fn:ident;)*) => {$(
        impl $trait for Score {
            #[inline]
            fn $fn(&mut self, rhs: Score) {
                self.0.$fn(rhs.0);
            }
        }

        impl $trait<i32> for Score {
            #[inline]
            fn $fn(&mut self, rhs: i32) {
                self.0.$fn(rhs);
            }
        }
    )*};
}

impl_score_ops! {
    Add, add;
    Sub, sub;
    Mul, mul;
    Div, div;
}

impl_score_assign_ops! {
    AddAssign, add_assign;
    SubAssign, sub_assign;
    MulAssign, mul_assign;
    DivAssign, div_assign;
}
