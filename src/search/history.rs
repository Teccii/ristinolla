use crate::board::Board;
use crate::search::{DEPTH_SCALE, W};
use crate::types::{Piece, Square};

/*----------------------------------------------------------------*/

pub const MAX_HISTORY: i32 = 16384;

#[inline]
fn gravity_with_decay<const MAX_BONUS: i32, const MAX_VALUE: i32>(
    entry: &mut i16,
    decay: i32,
    amount: i32,
) {
    let amount = amount.clamp(-MAX_BONUS, MAX_BONUS);
    let decay = (decay * amount.abs() / MAX_VALUE) as i16;
    *entry += amount as i16 - decay;
}

#[inline]
fn gravity<const MAX_BONUS: i32, const MAX_VALUE: i32>(entry: &mut i16, amount: i32) {
    gravity_with_decay::<MAX_BONUS, MAX_VALUE>(entry, *entry as i32, amount);
}

/*----------------------------------------------------------------*/

#[derive(Debug, Copy, Clone)]
pub struct MainEntry(i16);

#[derive(Debug, Copy, Clone)]
pub struct MainHistory {
    entries: [[MainEntry; Square::COUNT]; Piece::COUNT]
}

impl MainHistory {
    #[inline]
    pub fn bonus(depth: i32) -> i32 {
        let depth = depth as i64;
        let depth_scale = DEPTH_SCALE as i64;

        let base = W::main_bonus_base();
        let scale = W::main_bonus_scale1() * depth / depth_scale;
        let max = W::main_bonus_max();

        (base + scale).min(max) as i32
    }

    #[inline]
    pub fn malus(depth: i32) -> i32 {
        let depth = depth as i64;
        let depth_scale = DEPTH_SCALE as i64;

        let base = W::main_malus_base();
        let scale = W::main_malus_scale1() * depth / depth_scale;
        let max = W::main_malus_max();

        (base + scale).min(max) as i32
    }

    /*----------------------------------------------------------------*/

    #[inline]
    pub fn entry(&self, board: &Board, mv: Square) -> i32 {
        self.entries[board.stm()][mv].0 as i32
    }

    #[inline]
    pub fn entry_mut(&mut self, board: &Board, mv: Square) -> &mut i16 {
        &mut self.entries[board.stm()][mv].0
    }

    /*----------------------------------------------------------------*/

    #[inline]
    pub fn update(&mut self, board: &Board, depth: i32, mv: Square, bonus: bool) {
        let amount = if bonus {
            Self::bonus(depth)
        } else {
            -Self::malus(depth)
        };

        gravity::<MAX_HISTORY, MAX_HISTORY>(self.entry_mut(board, mv), amount);
    }
}

/*----------------------------------------------------------------*/


#[derive(Debug, Copy, Clone)]
pub struct History {
    main: MainHistory,
}

impl History {
    #[inline]
    pub fn score(&self, board: &Board, mv: Square) -> i32 {
        self.main.entry(board, mv)
    }

    /*----------------------------------------------------------------*/

    #[inline]
    pub fn update(
        &mut self,
        board: &Board,
        depth: i32,
        best_move: Square,
        moves: &[Square],
    ) {
        self.main.update(board, depth, best_move, true);
        for &mv in moves {
            self.main.update(board, depth, mv, false);
        }
    }
}
