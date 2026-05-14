use crate::board::Board;
use crate::types::{File, Piece, Rank, Square};

impl Board {
    #[inline]
    pub fn print(&self) {
        println!("╔═══╤═══╤═══╦═══╤═══╤═══╦═══╤═══╤═══╗");

        for &rank in Rank::ALL.iter().rev() {
            print!("║");

            for &file in File::ALL {
                let sq = Square::new(file, rank);
                let ch = match self.piece_on(sq) {
                    Some(Piece::X) => 'X',
                    Some(Piece::O) => 'O',
                    None => ' ',
                };

                print!(" {} ", ch);
                if matches!(file, File::C | File::F | File::I) {
                    print!("║");
                } else {
                    print!("│");
                }
            }

            println!();
            if matches!(rank, Rank::Seventh | Rank::Fourth) {
                println!("╠═══╪═══╪═══╬═══╪═══╪═══╬═══╪═══╪═══╣");
            } else if rank != Rank::First {
                println!("╟───┼───┼───╫───┼───┼───╫───┼───┼───╢");
            }
        }

        println!("╚═══╧═══╧═══╩═══╧═══╧═══╩═══╧═══╧═══╝");
        println!("Zobrist Key: {:#016X}", self.hash());
        println!("Side To Move: {:?}", self.stm());
    }
}
