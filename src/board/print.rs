use crate::{
    board::Board,
    types::{File, Piece, Rank, Square},
};
use colored::Colorize;

impl Board {
    #[inline]
    pub fn print(&self) {
        println!("╔═══╤═══╤═══╦═══╤═══╤═══╦═══╤═══╤═══╗");

        for &rank in Rank::ALL.iter().rev() {
            print!("║");

            for &file in File::ALL {
                let sq = Square::new(file, rank);
                let piece = if let Some(piece) = self.piece_on(sq) {
                    match piece {
                        Piece::X => String::from(char::from(piece)).bright_green(),
                        Piece::O => String::from(char::from(piece)).bright_blue(),
                    }
                    .to_string()
                } else {
                    String::from(" ")
                };

                print!(" {} ", piece);
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
        println!(
            "{}: {:#016X}",
            String::from("Zobrist Key").bright_green(),
            self.hash()
        );
        if let Some(mv) = self.prev_mv {
            println!("{}: {mv}", String::from("Previous Move").bright_green())
        }
        println!(
            "{}: {}",
            String::from("Side To Move").bright_green(),
            self.stm()
        );
    }
}
