use crate::{
    board::Board,
    engine::Engine,
    search::{SearchInfo, SearchLimit},
};
use std::{
    sync::atomic::Ordering,
    time::{Duration, Instant},
};

const BENCH_FENS: &[&str] = &["9/9/9/9/9/9/9/9/9/ x 0 -"];

impl Engine {
    #[inline]
    pub fn bench(&mut self, depth: u8) {
        let limits = vec![SearchLimit::Depth(depth)];
        let mut total_time = Duration::ZERO;
        let mut total_nodes = 0u64;

        for board in BENCH_FENS.iter().map(|&fen| Board::from_fen(fen).unwrap()) {
            self.pos.set_board(board.clone());
            self.searcher.newgame();

            let time = Instant::now();
            self.searcher.search(
                self.pos.clone(),
                limits.clone(),
                self.options,
                SearchInfo::None,
            );
            self.searcher.wait();

            total_time += time.elapsed();
            total_nodes += self.searcher.shared.nodes.load(Ordering::Relaxed);
        }

        let nanos = total_time.as_nanos();
        let nps = if nanos > 0 {
            (total_nodes as u128 * 1_000_000_000) / nanos
        } else {
            0
        };

        println!("nodes {total_nodes} time {total_time:.2?} nps {nps}");
    }
}
