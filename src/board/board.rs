use crate::{
    board::ZOBRIST,
    types::{Bitboard, Move, Piece, Square},
};
use enum_map::EnumMap;

#[derive(Copy, Clone, Default)]
pub struct Board {
    pieces: EnumMap<Piece, Bitboard>,
    hash: u64,
    stm: Piece,
}

impl Board {
    #[inline]
    pub fn occupied(&self) -> Bitboard {
        self.pieces[Piece::X] | self.pieces[Piece::O]
    }

    #[inline]
    pub fn pieces(&self, piece: Piece) -> Bitboard {
        self.pieces[piece]
    }

    #[inline]
    pub fn hash(&self) -> u64 {
        self.hash
    }

    #[inline]
    pub fn stm(&self) -> Piece {
        self.stm
    }

    /*----------------------------------------------------------------*/

    #[inline]
    pub fn piece_on(&self, sq: Square) -> Option<Piece> {
        if self.pieces[Piece::X].has(sq) {
            Some(Piece::X)
        } else if self.pieces[Piece::O].has(sq) {
            Some(Piece::O)
        } else {
            None
        }
    }

    /*----------------------------------------------------------------*/

    #[inline]
    pub fn make_move(&mut self, mv: Move) {
        self.xor_piece(mv.dest(), self.stm);
        self.toggle_stm();
    }

    /*----------------------------------------------------------------*/

    #[inline]
    fn xor_piece(&mut self, sq: Square, piece: Piece) {
        self.pieces[piece] ^= sq.bitboard();
        self.hash ^= ZOBRIST.pieces[piece][sq];
    }

    #[inline]
    fn toggle_stm(&mut self) {
        self.stm = !self.stm;
        self.hash ^= ZOBRIST.stm;
    }
}
