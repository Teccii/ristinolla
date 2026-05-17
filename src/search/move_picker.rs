use crate::{position::Position, search::History, types::Square};
use smallvec::{Array, SmallVec};

/*----------------------------------------------------------------*/

#[inline]
fn swap_pop<A: Array>(vec: &mut SmallVec<A>, index: usize) -> Option<A::Item> {
    let len = vec.len();

    if index >= len {
        return None;
    }

    vec.swap(index, len - 1);
    vec.pop()
}

#[inline]
fn select_next(moves: &[ScoredMove]) -> Option<usize> {
    moves
        .iter()
        .enumerate()
        .max_by_key(|(_, mv)| mv.1)
        .map(|(i, _)| i)
}

/*----------------------------------------------------------------*/

pub struct ScoredMove(pub Square, pub i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Stage {
    GenMoves,
    YieldMoves,
    Finished,
}

pub struct MovePicker {
    stage: Stage,
    moves: SmallVec<[ScoredMove; 32]>,
}

impl MovePicker {
    #[inline]
    pub fn new() -> Self {
        MovePicker {
            stage: Stage::GenMoves,
            moves: SmallVec::new(),
        }
    }

    #[inline]
    pub fn stage(&self) -> Stage {
        self.stage
    }

    #[inline]
    pub fn next(&mut self, pos: &mut Position, hist: &History) -> Option<ScoredMove> {
        if self.stage == Stage::GenMoves {
            for &mv in pos.board().gen_moves().iter() {
                self.moves.push(ScoredMove(mv, hist.score(pos.board(), mv)));
            }

            self.stage = Stage::YieldMoves;
        }

        if self.stage == Stage::YieldMoves {
            if let Some(index) = select_next(&self.moves) {
                return swap_pop(&mut self.moves, index);
            }

            self.stage = Stage::Finished;
        }

        None
    }
}
