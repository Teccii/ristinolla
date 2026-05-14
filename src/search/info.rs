use crate::search::PrincipalVariation;
use crate::{
    score::Score,
    search::{SharedData, ThreadData},
    types::Square,
};

#[derive(Debug, Clone)]
pub enum SearchInfo {
    Ugi { minimal: bool },
    None,
}

impl SearchInfo {
    #[inline]
    pub fn update(
        &mut self,
        thread: &ThreadData,
        shared: &SharedData,
        pv: &PrincipalVariation,
        score: Score,
        depth: u8,
        last: bool,
    ) {
        let SearchInfo::Ugi { minimal } = self else {
            return;
        };

        if *minimal && !last {
            return;
        }

        let nodes = thread.nodes.global();
        let time = shared.time_man.elapsed();
        let nps = if time.as_nanos() > 0 {
            (nodes as u128 * 1_000_000_000) / time.as_nanos()
        } else {
            0
        };

        println!(
            "info depth {depth} seldepth {} score {score} time {} nodes {nodes} nps {nps} pv {}",
            thread.sel_depth,
            time.as_millis(),
            pv.display()
        );
    }

    #[inline]
    pub fn best_move(&mut self, best_move: Square) {
        let SearchInfo::Ugi { minimal } = self else {
            return;
        };

        println!("bestmove {best_move}");
    }
}
