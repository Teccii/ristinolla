use crate::{board::Board, types::Square};
use arrayvec::ArrayVec;
use std::ops::{Deref, DerefMut};

/*----------------------------------------------------------------*/

#[derive(Debug, Clone)]
pub struct MoveList(ArrayVec<Square, { Square::COUNT }>);

impl MoveList {
    #[inline]
    pub fn new() -> Self {
        MoveList(ArrayVec::new())
    }
}

impl Deref for MoveList {
    type Target = ArrayVec<Square, { Square::COUNT }>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MoveList {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/*----------------------------------------------------------------*/

impl Board {
    #[inline]
    pub fn gen_moves(&self) -> MoveList {
        if self.terminal_state().is_some() {
            return MoveList::new();
        }

        let mut moves = MoveList::new();
        if let Some(mv) = self.prev_mv {
            let index = mv.indices().1;
            if self.small[index].terminal_state().is_none() {
                for &sq in Square::ALL {
                    if sq.indices().0 == index && self.piece_on(sq).is_none() {
                        moves.push(sq);
                    }
                }

                return moves;
            }
        }

        let valid = self.small.map(|ttt| ttt.terminal_state().is_none());
        for &sq in Square::ALL {
            let index = sq.indices().0;
            if valid[index] && self.piece_on(sq).is_none() {
                moves.push(sq);
            }
        }

        moves
    }

    #[inline]
    pub fn is_legal(&self, mv: Square) -> bool {
        if self.terminal_state().is_some() {
            return false;
        }

        if let Some(prev_mv) = self.prev_mv {
            let index = prev_mv.indices().1;
            if self.small[index].terminal_state().is_none() {
                return mv.indices().0 == index && self.piece_on(mv).is_none();
            }
        }

        self.small[mv.indices().0].terminal_state().is_none() && self.piece_on(mv).is_none()
    }
}
