use crate::board::TerminalState;
use crate::{
    board::Board,
    types::{File, Piece, Rank, Square},
};

impl Board {
    pub fn from_fen(fen: &str) -> Option<Board> {
        let mut parts = fen.trim().split_ascii_whitespace();
        let pieces = parts.next()?;
        let stm = parts.next()?;
        let ply = parts.next()?;
        let prev_move = parts.next()?;

        if parts.next().is_some() {
            return None;
        }

        if stm.len() != 1 {
            return None;
        }

        let mut board = Board::default();
        board.ply = ply.parse::<u8>().ok()?.max(0);

        match stm.chars().next() {
            Some('x') | Some('X') => {}
            Some('o') | Some('O') => board.toggle_stm(),
            _ => return None,
        }

        if let Some(mv) = prev_move.parse::<Square>().ok() {
            board.xor_move(mv);
        }

        for (rank, row) in pieces.rsplit('/').enumerate() {
            let rank = Rank::try_index(rank)?;
            let mut file = 0;

            for p in row.chars() {
                if let Some(empty) = p.to_digit(10) {
                    file += empty as usize;
                } else {
                    let piece = match p {
                        'x' | 'X' => Some(Piece::X),
                        'o' | 'O' => Some(Piece::O),
                        _ => None,
                    };
                    let sq = Square::new(File::try_index(file)?, rank);

                    if let Some(piece) = piece {
                        board.xor_piece(piece, sq);
                    }

                    file += 1;
                }
            }

            if file != File::COUNT {
                return None;
            }
        }

        for i in 0..9 {
            if let Some(TerminalState::Victory(piece)) = board.small[i].terminal_state() {
                board.large.set(piece, i);
            }
        }

        Some(board)
    }
}
