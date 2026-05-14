use crate::{
    board::{Board, TerminalState},
    types::{Piece, Square},
};

#[derive(Clone)]
pub struct Position {
    current: Board,
    boards: Vec<Board>,
    moves: Vec<Square>,
}

impl Position {
    #[inline]
    pub fn new(board: Board) -> Position {
        Position {
            current: board,
            boards: Vec::new(),
            moves: Vec::new(),
        }
    }

    #[inline]
    pub fn set_board(&mut self, board: Board) {
        self.current = board;
        self.boards.clear();
        self.moves.clear();
    }

    /*----------------------------------------------------------------*/

    #[inline]
    pub fn board(&self) -> &Board {
        &self.current
    }

    #[inline]
    pub fn prev_move(&self, ply: usize) -> Option<Square> {
        self.moves.len().checked_sub(ply).map(|i| self.moves[i])
    }

    #[inline]
    pub fn terminal_state(&self) -> Option<TerminalState> {
        self.current.terminal_state()
    }

    #[inline]
    pub fn stm(&self) -> Piece {
        self.current.stm()
    }

    #[inline]
    pub fn hash(&self) -> u64 {
        self.current.hash()
    }

    /*----------------------------------------------------------------*/

    #[inline]
    pub fn make_move(&mut self, mv: Square) {
        self.moves.push(mv);
        self.boards.push(self.current.clone());
        self.current.make_move(mv);
    }

    #[inline]
    pub fn unmake_move(&mut self) {
        self.current = self.boards.pop().unwrap();
        self.moves.pop().unwrap();
    }
}
