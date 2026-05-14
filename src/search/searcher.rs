use crate::{
    board::MoveList,
    engine::EngineOptions,
    position::Position,
    search::{SearchInfo, SearchLimit, SharedData, thread_loop},
    util::{Sender, channel},
};
use std::{
    sync::{Arc, atomic::*},
    thread::JoinHandle,
};

/*----------------------------------------------------------------*/

#[derive(Clone)]
pub enum ThreadCommand {
    Search {
        pos: Position,
        root_moves: MoveList,
        options: EngineOptions,
        info: SearchInfo,
    },
    SetShared(Arc<SharedData>),
    NewGame,
    Quit,
}

/*----------------------------------------------------------------*/

pub struct Searcher {
    pub shared: Arc<SharedData>,
    command_sender: Sender<ThreadCommand>,
    search_threads: Vec<JoinHandle<()>>,
}

impl Default for Searcher {
    #[inline]
    fn default() -> Self {
        let shared = Arc::new(SharedData::default());
        let (tx, mut rx) = channel(1);
        let search_thread = std::thread::spawn({
            let shared = shared.clone();

            move || {
                if std::panic::catch_unwind(move || thread_loop(0, rx.next().unwrap(), shared))
                    .is_err()
                {
                    std::process::exit(1);
                }
            }
        });

        Searcher {
            shared,
            command_sender: tx,
            search_threads: vec![search_thread],
        }
    }
}

impl Searcher {
    #[inline]
    pub fn is_searching(&self) -> bool {
        self.shared.num_searching.load(Ordering::Relaxed) != 0
    }

    /*----------------------------------------------------------------*/

    pub fn search(
        &mut self,
        pos: Position,
        limits: Vec<SearchLimit>,
        options: EngineOptions,
        info: SearchInfo,
    ) {
        self.shared.num_searching.fetch_add(1, Ordering::Relaxed);
        self.shared.time_man.init(
            pos.stm(),
            &limits,
            options.move_overhead,
            options.soft_target,
        );

        let root_moves = limits
            .iter()
            .find_map(|l| match l {
                SearchLimit::SearchMoves(moves) => Some(moves.clone()),
                _ => None,
            })
            .unwrap_or_else(|| pos.board().gen_moves());
        self.command_sender.send(ThreadCommand::Search {
            pos,
            root_moves,
            options,
            info,
        });
    }

    #[inline]
    pub fn set_threads(&mut self, threads: u32) {
        assert!(
            !self.is_searching(),
            "Called `set_threads() while searching"
        );
        assert!(threads > 0, "Threads must be greater than 0");

        self.command_sender.send(ThreadCommand::Quit);
        self.search_threads
            .drain(..)
            .for_each(|t| t.join().unwrap());

        let (tx, rx) = channel(threads);
        self.search_threads = rx
            .enumerate()
            .map(|(i, rx)| {
                std::thread::spawn({
                    let shared = self.shared.clone();
                    move || {
                        if std::panic::catch_unwind(move || thread_loop(i, rx, shared)).is_err() {
                            std::process::exit(1);
                        }
                    }
                })
            })
            .collect();
        self.command_sender = tx;
    }

    #[inline]
    pub fn newgame(&mut self) {
        assert!(!self.is_searching(), "Called `newgame()` while searching");
        self.command_sender.send(ThreadCommand::NewGame);
    }

    #[inline]
    pub fn wait(&self) {
        let mut num_searching = self.shared.num_searching.load(Ordering::Relaxed);
        while num_searching != 0 {
            atomic_wait::wait(&self.shared.num_searching, num_searching);
            num_searching = self.shared.num_searching.load(Ordering::Relaxed);
        }
    }

    #[inline]
    pub fn stop(&self) {
        assert!(self.is_searching(), "Called `stop()` while not searching");
        self.shared.time_man.set_stop(true);
    }

    #[inline]
    pub fn quit(&mut self) {
        self.shared.time_man.set_stop(true);
        self.command_sender.send(ThreadCommand::Quit);
        self.search_threads
            .drain(..)
            .for_each(|t| t.join().unwrap());
    }
}
