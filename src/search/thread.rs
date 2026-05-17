use crate::board::MoveList;
use crate::search::{PrincipalVariation, TimeManager};
use crate::{
    search::{MAX_PLY, SearchStack, ThreadCommand, id_loop},
    util::{BatchedAtomicCounter, Receiver},
};
use std::sync::{Arc, atomic::*};
use crate::search::history::History;
/*----------------------------------------------------------------*/

pub const MAX_THREADS: u32 = 1024;

pub struct SharedData {
    pub num_searching: AtomicU32,
    pub time_man: TimeManager,
    pub nodes: Arc<AtomicU64>,
}

impl Default for SharedData {
    #[inline]
    fn default() -> Self {
        SharedData {
            num_searching: AtomicU32::new(0),
            time_man: TimeManager::new(),
            nodes: Arc::new(AtomicU64::new(0)),
        }
    }
}

/*----------------------------------------------------------------*/

#[derive(Clone)]
pub struct ThreadData {
    pub id: usize,
    pub sel_depth: usize,
    pub hist: Box<History>,
    pub root_moves: MoveList,
    pub root_pv: PrincipalVariation,
    pub stack: Vec<SearchStack>,
    pub nodes: BatchedAtomicCounter,
    pub stop: bool,
}

impl ThreadData {
    #[inline]
    pub fn new(id: usize, nodes: Arc<AtomicU64>) -> Self {
        ThreadData {
            id,
            sel_depth: 0,
            hist: unsafe { Box::new_zeroed().assume_init() },
            root_moves: MoveList::new(),
            root_pv: PrincipalVariation::default(),
            stack: vec![SearchStack::default(); MAX_PLY + 1],
            nodes: BatchedAtomicCounter::new(nodes),
            stop: false,
        }
    }

    #[inline]
    pub fn reset(&mut self) {
        self.nodes.reset();
        self.root_moves.clear();
        self.root_pv = PrincipalVariation::default();
        self.stack = vec![SearchStack::default(); MAX_PLY + 1];
        self.sel_depth = 0;
        self.stop = false;
    }
}

/*----------------------------------------------------------------*/

pub fn thread_loop(id: usize, mut rx: Receiver<ThreadCommand>, mut shared: Arc<SharedData>) {
    let mut thread = ThreadData::new(id, shared.nodes.clone());

    loop {
        match rx.recv(|cmd| cmd.clone()) {
            ThreadCommand::Search {
                pos,
                root_moves,
                options: _options,
                info,
            } => {
                thread.reset();
                thread.root_moves = root_moves;
                shared.num_searching.fetch_add(1, Ordering::Relaxed);

                id_loop(pos, &mut thread, &shared, info);
            }
            ThreadCommand::SetShared(new_shared) => {
                shared = new_shared;
                thread.nodes = BatchedAtomicCounter::new(shared.nodes.clone());
            }
            ThreadCommand::NewGame => {
                thread.hist = unsafe { Box::new_zeroed().assume_init() };
            }
            ThreadCommand::Quit => return,
        }
    }
}
