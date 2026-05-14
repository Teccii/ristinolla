use crate::search::{DEFAULT_OVERHEAD, MAX_THREADS, SearchInfo, SearchLimit};
use crate::types::Square;
use crate::ugi::UgiParseError;
use crate::{board::Board, position::Position, search::Searcher, ugi::UgiCommand};
use std::time::{Duration, Instant};
/*----------------------------------------------------------------*/

pub const ENGINE_VERSION: &str = env!("CARGO_PKG_VERSION");

fn perft<const BULK: bool>(board: &Board, depth: u8) -> u64 {
    if depth == 0 {
        return 1;
    }

    let mut nodes = 0;
    let move_list = board.gen_moves();

    if BULK && depth == 1 {
        nodes += move_list.len() as u64;
    } else {
        for &mv in move_list.iter() {
            let mut board = board.clone();
            board.make_move(mv);

            nodes += perft::<BULK>(&board, depth - 1);
        }
    }

    nodes
}

/*----------------------------------------------------------------*/

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Abort {
    Yes,
    No,
}

#[derive(Copy, Clone)]
pub struct EngineOptions {
    pub minimal: bool,
    pub soft_target: bool,
    pub move_overhead: u64,
}

impl Default for EngineOptions {
    #[inline]
    fn default() -> Self {
        EngineOptions {
            minimal: false,
            soft_target: false,
            move_overhead: 50,
        }
    }
}

/*----------------------------------------------------------------*/

pub struct Engine {
    pub pos: Position,
    pub searcher: Searcher,
    pub options: EngineOptions,
}

impl Engine {
    #[inline]
    pub fn new() -> Engine {
        Engine {
            pos: Position::new(Board::default()),
            searcher: Searcher::default(),
            options: EngineOptions::default(),
        }
    }

    #[inline]
    pub fn handle(&mut self, input: &str) -> Abort {
        let cmd = match UgiCommand::parse(input, self.pos.board()) {
            Ok(cmd) => cmd,
            Err(e) => {
                println!("info string {e}");
                return Abort::No;
            }
        };

        match cmd {
            UgiCommand::Ugi => self.ugi(),
            UgiCommand::NewGame => self.searcher.newgame(),
            UgiCommand::IsReady => println!("readyok"),
            UgiCommand::Display => self.display(),
            UgiCommand::Position { board, moves } => self.set_pos(board, moves),
            UgiCommand::Search(limits) => self.search(limits),
            UgiCommand::Perft { depth, bulk } => self.perft(depth, bulk),
            UgiCommand::SplitPerft { depth, bulk } => self.splitperft(depth, bulk),
            UgiCommand::SetOption { name, value } => self.set_option(name, value),
            UgiCommand::Bench { depth } => self.bench(depth),
            UgiCommand::Wait => self.wait(),
            UgiCommand::Stop => self.stop(),
            UgiCommand::Quit => return self.quit(),
        }

        Abort::No
    }

    #[inline]
    fn ugi(&self) {
        println!("id name Ristinolla v{ENGINE_VERSION}-dev");
        println!("id author Tecci");
        println!("option name Threads type spin default 1 min 1 max {MAX_THREADS}");
        println!("option name Minimal type check default false");
        println!("option name SoftTarget type check default false");
        println!("option name MoveOverhead type spin default {DEFAULT_OVERHEAD} min 0 max 5000");
        println!("ugiok");
    }

    #[inline]
    fn display(&self) {
        self.pos.board().print();
    }

    #[inline]
    fn set_pos(&mut self, board: Board, moves: Vec<Square>) {
        self.pos.set_board(board);
        for mv in moves {
            self.pos.make_move(mv);
        }
    }

    #[inline]
    fn search(&mut self, limits: Vec<SearchLimit>) {
        if self.searcher.is_searching() {
            println!("info string Already Searching");
            return;
        }
        
        self.searcher.search(
            self.pos.clone(),
            limits,
            self.options,
            SearchInfo::Ugi {
                minimal: self.options.minimal,
            },
        );
    }

    #[inline]
    fn perft(&mut self, depth: u8, bulk: bool) {
        let board = self.pos.board().clone();
        let time = Instant::now();
        let nodes = if bulk {
            perft::<true>(&board, depth)
        } else {
            perft::<false>(&board, depth)
        };
        let elapsed = time.elapsed();
        let nanos = elapsed.as_nanos();
        let nps = if nanos > 0 {
            (nodes as u128 * 1_000_000_000) / nanos
        } else {
            0
        };

        println!("nodes {nodes} time {elapsed:.2?} nps {nps}");
    }

    #[inline]
    fn splitperft(&mut self, depth: u8, bulk: bool) {
        if depth == 0 {
            return;
        }

        let board = self.pos.board().clone();
        let mut perft_data = Vec::new();
        let mut total_time = Duration::ZERO;
        let mut total_nodes = 0u64;

        for &mv in board.gen_moves().iter() {
            let mut board = board.clone();
            board.make_move(mv);

            let time = Instant::now();
            let nodes = if bulk {
                perft::<true>(&board, depth)
            } else {
                perft::<false>(&board, depth)
            };

            total_time += time.elapsed();
            total_nodes += nodes;

            perft_data.push((mv, nodes));
        }

        for (mv, nodes) in perft_data {
            println!("{mv:<5}: {nodes}");
        }

        let nanos = total_time.as_nanos();
        let nps = if nanos > 0 {
            (total_nodes as u128 * 1_000_000_000) / nanos
        } else {
            0
        };

        println!("\nnodes {total_nodes} time {total_time:.2?} nps {nps}");
    }

    #[inline]
    fn set_option(&mut self, name: String, value: String) {
        match name.as_str() {
            "Threads" => {
                if self.searcher.is_searching() {
                    println!("info string Not Allowed to set Threads while Searching");
                    return;
                }

                let value = match value.parse::<u32>() {
                    Ok(value) => value,
                    Err(e) => {
                        println!("info string {:?}", UgiParseError::InvalidInteger(e));
                        return;
                    }
                };

                if value == 0 || value > MAX_THREADS {
                    println!("info string Invalid Number of Threads: `{value}`");
                    return;
                }

                self.searcher.set_threads(value);
                println!("info string Set Threads to {value}");
            }
            "Minimal" => {
                let value = match value.parse::<bool>() {
                    Ok(value) => value,
                    Err(e) => {
                        println!("info string {:?}", UgiParseError::InvalidBool(e));
                        return;
                    }
                };

                self.options.minimal = value;
                println!("info string Set Minimal to {value}");
            }
            "MoveOverhead" => {
                let value = match value.parse::<u64>() {
                    Ok(value) => value,
                    Err(e) => {
                        println!("info string {:?}", UgiParseError::InvalidInteger(e));
                        return;
                    }
                };

                if value > 5000 {
                    println!("info string Invalid MoveOverhead value: `{value}`");
                    return;
                }

                self.options.move_overhead = value;
                println!("info string Set MoveOverhead to {value}");
            }
            "SoftTarget" => {
                let value = match value.parse::<bool>() {
                    Ok(value) => value,
                    Err(e) => {
                        println!("info string {:?}", UgiParseError::InvalidBool(e));
                        return;
                    }
                };

                self.options.soft_target = value;
                println!("info string Set SoftTarget to {value}");
            }
            _ => println!("info string Unknown Option: `{name}`"),
        }
    }

    #[inline]
    fn wait(&self) {
        if !self.searcher.is_searching() {
            println!("info string Not Searching");
        } else {
            println!("info string Waiting for Search to Stop...");
            self.searcher.wait();
            println!("info string Searcher Stopped");
        }
    }

    #[inline]
    fn stop(&self) {
        if self.searcher.is_searching() {
            self.searcher.stop();
            self.searcher.wait();

            println!("info string Searcher Stopped");
        } else {
            println!("info string Not Searching");
        }
    }

    #[inline]
    fn quit(&mut self) -> Abort {
        self.searcher.quit();
        Abort::Yes
    }
}
