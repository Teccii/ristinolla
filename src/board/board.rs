use crate::{
    board::ZOBRIST,
    types::{Piece, Square},
};
use enum_map::EnumMap;
use std::cmp::Ordering;

/*----------------------------------------------------------------*/

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TerminalState {
    Victory(Piece),
    Draw,
}

/*----------------------------------------------------------------*/

#[derive(Debug, Copy, Clone, Default)]
pub struct TicTacToe(EnumMap<Piece, u16>);

impl TicTacToe {
    #[inline]
    pub fn set(&mut self, piece: Piece, index: usize) {
        self.0[piece] |= 1 << index;
    }

    #[inline]
    pub fn has(&self, piece: Piece, index: usize) -> bool {
        (self.0[piece] & (1 << index)) != 0
    }

    #[inline]
    pub fn terminal_state(&self) -> Option<TerminalState> {
        const TERMINAL_CHECKS: &[u16; 8] = &[
            0b111,
            0b111000,
            0b111000000,
            0b1001001,
            0b10010010,
            0b100100100,
            0b100010001,
            0b1010100,
        ];

        for &check in TERMINAL_CHECKS {
            for &piece in Piece::ALL {
                if self.0[piece] & check == check {
                    return Some(TerminalState::Victory(piece));
                }
            }
        }

        if self.0[Piece::X] | self.0[Piece::O] == 0b111111111 {
            return Some(TerminalState::Draw);
        }

        None
    }
}

/*----------------------------------------------------------------*/

#[derive(Debug, Copy, Clone, Default)]
pub struct Board {
    pub(super) prev_mv: Option<Square>,
    pub(super) small: [TicTacToe; 9],
    pub(super) large: TicTacToe,
    pub(super) stm: Piece,
    pub(super) hash: u64,
    pub(super) ply: u8,
}

impl Board {
    #[inline]
    pub fn stm(&self) -> Piece {
        self.stm
    }

    #[inline]
    pub fn hash(&self) -> u64 {
        self.hash
    }

    #[inline]
    pub fn ply(&self) -> u8 {
        self.ply
    }

    /*----------------------------------------------------------------*/

    #[inline]
    pub fn terminal_state(&self) -> Option<TerminalState> {
        self.large.terminal_state().or_else(|| {
            let mut won_boards: EnumMap<Piece, usize> = EnumMap::default();

            for i in 0..9 {
                if let Some(terminal_state) = self.small[i].terminal_state() {
                    if let TerminalState::Victory(p) = terminal_state {
                        won_boards[p] += 1;
                    }
                } else {
                    return None;
                }

                if self.small[i].terminal_state().is_none() {
                    return None;
                }
            }

            match won_boards[Piece::X].cmp(&won_boards[Piece::O]) {
                Ordering::Greater => Some(TerminalState::Victory(Piece::X)),
                Ordering::Less => Some(TerminalState::Victory(Piece::O)),
                Ordering::Equal => Some(TerminalState::Draw),
            }
        })
    }

    #[inline]
    pub fn piece_on(&self, sq: Square) -> Option<Piece> {
        let (board, sq) = sq.indices();
        if self.small[board].has(Piece::X, sq) {
            Some(Piece::X)
        } else if self.small[board].has(Piece::O, sq) {
            Some(Piece::O)
        } else {
            None
        }
    }

    /*----------------------------------------------------------------*/

    #[inline]
    pub fn make_move(&mut self, mv: Square) {
        if let Some(mv) = self.prev_mv {
            self.xor_move(mv);
        }
        self.xor_piece(self.stm, mv);

        let index = mv.indices().0;
        if let Some(TerminalState::Victory(piece)) = self.small[index].terminal_state() {
            self.large.set(piece, index);
        }

        self.xor_move(mv);
        self.toggle_stm();
    }

    /*----------------------------------------------------------------*/

    #[inline]
    pub(super) fn xor_piece(&mut self, piece: Piece, sq: Square) {
        self.hash ^= ZOBRIST.pieces[piece][sq];

        let indices = sq.indices();
        self.small[indices.0].set(piece, indices.1);
    }

    #[inline]
    pub(super) fn xor_move(&mut self, mv: Square) {
        let index = mv.indices().1;
        if self.small[index].terminal_state().is_none() {
            self.hash ^= ZOBRIST.subboard[index];
        }
        self.prev_mv = Some(mv);
    }

    #[inline]
    pub(super) fn toggle_stm(&mut self) {
        self.hash ^= ZOBRIST.stm;
        self.stm = !self.stm;
    }
}
