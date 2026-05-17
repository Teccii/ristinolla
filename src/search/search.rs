use crate::{
    board::{Board, TerminalState},
    position::Position,
    score::Score,
    search::{SearchInfo, SharedData, ThreadData},
    types::Square,
};
use std::{fmt::Write, sync::atomic::*};
use smallvec::SmallVec;
use crate::search::{MovePicker, ScoredMove};

pub const MAX_PLY: usize = 81;
pub const MAX_DEPTH: usize = 255;
pub const MAX_FRAC_DEPTH: i32 = MAX_DEPTH as i32 * DEPTH_SCALE;
pub const DEPTH_SCALE: i32 = 1024;

pub fn id_loop(
    mut pos: Position,
    thread: &mut ThreadData,
    shared: &SharedData,
    mut info: SearchInfo,
) {
    let mut score = Score::NONE;
    let mut best_move: Option<Square> = None;
    let mut completed_depth = 0;
    let mut depth = 1;

    'id: loop {
        thread.sel_depth = 0;

        let new_score = search::<Root>(&mut pos, thread, shared, depth * DEPTH_SCALE, 0, -Score::INF, Score::INF);
        thread.nodes.flush();

        if depth > 1 && thread.stop {
            break 'id;
        }

        completed_depth += 1;
        depth += 1;

        thread.root_pv = thread.stack[0].pv.clone();
        best_move = thread.root_pv.moves[0];
        score = new_score;

        if thread.id == 0 {
            info.update(
                thread,
                shared,
                &thread.root_pv,
                score,
                completed_depth,
                false,
            );
        }

        if thread.id == 0
            && shared
                .time_man
                .stop_id(completed_depth, thread.nodes.global())
        {
            shared.time_man.set_stop(true);
            break 'id;
        }
    }

    let last_thread = shared.num_searching.fetch_sub(1, Ordering::Relaxed) == 2;
    if last_thread && thread.id != 0 {
        atomic_wait::wake_all(&shared.num_searching);
    }

    if thread.id == 0 {
        if !last_thread {
            let mut num_searching = shared.num_searching.load(Ordering::Relaxed);
            while num_searching != 1 {
                atomic_wait::wait(&shared.num_searching, num_searching);
                num_searching = shared.num_searching.load(Ordering::Relaxed);
            }
        }
        shared.num_searching.store(0, Ordering::Relaxed);
    }

    if thread.id == 0 {
        info.update(
            thread,
            shared,
            &thread.root_pv,
            score,
            completed_depth,
            true,
        );
        info.best_move(best_move.unwrap());

        atomic_wait::wake_all(&shared.num_searching);
    }
}

/*----------------------------------------------------------------*/

pub trait NodeType {
    const PV: bool;
    const ROOT: bool;
    type Next: NodeType;
}

pub struct Root;
pub struct PV;
pub struct NonPV;

impl NodeType for Root {
    const PV: bool = true;
    const ROOT: bool = true;
    type Next = PV;
}

impl NodeType for PV {
    const PV: bool = true;
    const ROOT: bool = false;
    type Next = PV;
}

impl NodeType for NonPV {
    const PV: bool = false;
    const ROOT: bool = false;
    type Next = NonPV;
}

/*----------------------------------------------------------------*/

#[derive(Clone, Default)]
pub struct SearchStack {
    pub pv: PrincipalVariation,
}

#[derive(Clone)]
pub struct PrincipalVariation {
    pub moves: [Option<Square>; MAX_PLY + 1],
    pub len: usize,
}

impl PrincipalVariation {
    #[inline]
    pub fn update(&mut self, mv: Square, child_pv: &PrincipalVariation) {
        self.moves[0] = Some(mv);
        self.len = child_pv.len + 1;
        self.moves[1..self.len].copy_from_slice(&child_pv.moves[..child_pv.len]);
    }

    #[inline]
    pub fn display(&self) -> String {
        let mut output = String::new();
        if self.len != 0 {
            for &mv in self.moves[..self.len].iter() {
                if let Some(mv) = mv {
                    write!(output, "{mv} ").unwrap();
                } else {
                    break;
                }
            }
        }

        output
    }
}

impl Default for PrincipalVariation {
    #[inline]
    fn default() -> Self {
        PrincipalVariation {
            moves: [None; MAX_PLY + 1],
            len: 0,
        }
    }
}

/*----------------------------------------------------------------*/

fn eval(board: &Board) -> Score {
    #[rustfmt::skip]
    const PSQT: [i32; Square::COUNT] = [
        10, 5,  10, 10, 5,  10, 10, 5,  10,
        5,  20, 5,  5,  20, 5,  5,  20, 5,
        10, 5,  10, 10, 5,  10, 10, 5,  10,
        10, 5,  10, 10, 5,  10, 10, 5,  10,
        5,  20, 5,  5,  20, 5,  5,  20, 5,
        10, 5,  10, 10, 5,  10, 10, 5,  10,
        10, 5,  10, 10, 5,  10, 10, 5,  10,
        5,  20, 5,  5,  20, 5,  5,  20, 5,
        10, 5,  10, 10, 5,  10, 10, 5,  10,
    ];

    let mut score = Score(0);
    let stm = board.stm();

    for &sq in Square::ALL {
        match board.piece_on(sq) {
            Some(p) if p == stm => score += PSQT[sq],
            Some(p) if p != stm => score -= PSQT[sq],
            _ => {}
        }
    }

    score
}

/*----------------------------------------------------------------*/

pub fn search<Node: NodeType>(
    pos: &mut Position,
    thread: &mut ThreadData,
    shared: &SharedData,
    depth: i32,
    ply: u8,
    mut alpha: Score,
    beta: Score,
) -> Score {
    if !Node::ROOT && (thread.stop || shared.time_man.stop_search(thread)) {
        if thread.id == 0 {
            shared.time_man.set_stop(true);
        }
        thread.stop = true;
        return Score(0);
    }

    if !Node::ROOT {
        thread.nodes.inc();
        thread.sel_depth = thread.sel_depth.max(ply as usize);

        if let Some(terminal) = pos.terminal_state() {
            return match terminal {
                TerminalState::Victory(_) => Score::mated(ply),
                TerminalState::Draw => Score(0),
            };
        }

        if depth <= 0 {
            return eval(pos.board());
        }
    }

    let mut best_move = None;
    let mut best_score = Score::NONE;
    let mut move_picker = MovePicker::new();
    let mut move_count = 0;

    let mut moves: SmallVec<[Square; 32]> = SmallVec::new();
    while let Some(ScoredMove(mv, _)) = move_picker.next(pos, &thread.hist) {
        if Node::ROOT && !thread.root_moves.contains(&mv) {
            continue;
        }

        pos.make_move(mv);
        let score = -search::<Node::Next>(pos, thread, shared, depth - DEPTH_SCALE, ply + 1, -beta, -alpha);
        pos.unmake_move();
        move_count += 1;

        if Node::ROOT && move_count == 1 {
            let (parent, child) = thread.stack.split_at_mut(ply as usize + 1);
            let (parent, child) = (parent.last_mut().unwrap(), child.first().unwrap());
            parent.pv.update(mv, &child.pv);
        }

        if thread.stop {
            return Score(0);
        }

        if score > best_score {
            best_score = score;
        }

        if score > alpha {
            let (parent, child) = thread.stack.split_at_mut(ply as usize + 1);
            let (parent, child) = (parent.last_mut().unwrap(), child.first().unwrap());
            parent.pv.update(mv, &child.pv);

            best_move = Some(mv);
            alpha = score;
        }

        if score >= beta {
            thread.hist.update(pos.board(), depth, best_move.unwrap(), &moves);
            break;
        }

        if best_move != Some(mv) {
            moves.push(mv);
        }
    }

    best_score
}
