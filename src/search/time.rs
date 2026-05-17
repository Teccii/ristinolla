use crate::{
    board::MoveList,
    search::{MAX_DEPTH, ThreadData, W},
    types::Piece,
    util::AtomicInstant,
};
use enum_map::enum_map;
use std::{
    sync::atomic::*,
    time::{Duration, Instant},
};

pub const DEFAULT_OVERHEAD: u64 = 50;

#[derive(Debug, Clone)]
pub enum SearchLimit {
    SearchMoves(MoveList),
    XTime(u64),
    OTime(u64),
    XInc(u64),
    OInc(u64),
    MoveTime(u64),
    MovesToGo(u16),
    Nodes(u64),
    Depth(u8),
}

/*----------------------------------------------------------------*/

pub struct TimeManager {
    start: AtomicInstant,
    infinite: AtomicBool,
    check_time: AtomicBool,
    stop: AtomicU32,

    base_time: AtomicU64,
    soft_time: AtomicU64,
    hard_time: AtomicU64,

    soft_nodes: AtomicU64,
    hard_nodes: AtomicU64,
    depth: AtomicU8,
}

impl TimeManager {
    #[inline]
    pub fn new() -> Self {
        TimeManager {
            start: AtomicInstant::now(),
            infinite: AtomicBool::new(true),
            check_time: AtomicBool::new(false),
            stop: AtomicU32::new(0),

            base_time: AtomicU64::new(0),
            soft_time: AtomicU64::new(0),
            hard_time: AtomicU64::new(0),

            soft_nodes: AtomicU64::new(u64::MAX),
            hard_nodes: AtomicU64::new(u64::MAX),
            depth: AtomicU8::new(MAX_DEPTH as u8),
        }
    }

    #[inline]
    pub fn init(&self, stm: Piece, limits: &[SearchLimit], overhead: u64, soft_target: bool) {
        self.set_stop(false);

        let mut inc = enum_map! { _ => 0 };
        let mut time = enum_map! { _ => u64::MAX };
        let mut moves_to_go = None;
        let mut move_time = None;
        let mut nodes = u64::MAX;
        let mut depth = MAX_DEPTH as u8;
        let mut infinite = true;
        let mut check_time = false;

        for limit in limits {
            use SearchLimit::*;

            match *limit {
                SearchMoves(_) => {}
                XTime(t) => time[Piece::X] = t,
                OTime(t) => time[Piece::O] = t,
                XInc(i) => inc[Piece::X] = i,
                OInc(i) => inc[Piece::O] = i,
                MoveTime(t) => move_time = Some(t),
                MovesToGo(n) => moves_to_go = Some(n),
                Depth(d) => depth = depth.min(d),
                Nodes(n) => nodes = nodes.min(n),
            }

            if matches!(
                limit,
                XTime(..) | OTime(..) | MoveTime(..) | Depth(..) | Nodes(..)
            ) {
                infinite = false;
            }

            if matches!(limit, XTime(..) | OTime(..) | MoveTime(..)) {
                check_time = true;
            }
        }

        self.infinite.store(infinite, Ordering::Relaxed);
        self.check_time.store(check_time, Ordering::Relaxed);

        self.depth.store(depth, Ordering::Relaxed);
        self.soft_nodes.store(nodes, Ordering::Relaxed);
        if soft_target {
            self.hard_nodes
                .store(nodes.saturating_mul(2000), Ordering::Relaxed);
        } else {
            self.hard_nodes.store(nodes, Ordering::Relaxed);
        }

        if let Some(time) = move_time {
            self.base_time.store(time, Ordering::Relaxed);
            self.soft_time.store(time, Ordering::Relaxed);

            if soft_target {
                self.hard_time
                    .store(time.saturating_mul(2).max(time + 2000), Ordering::Relaxed);
            } else {
                self.hard_time.store(time, Ordering::Relaxed);
            }
        } else if let Some(moves_to_go) = moves_to_go {
            let (time, inc) = (time[stm].saturating_sub(overhead), inc[stm]);
            let hard_time =
                ((time as f64 / (W::hard_time_div() as f64 / 4096.0)) as u64 + inc).min(time);
            let soft_time = (time / moves_to_go as u64 + inc).min(hard_time);

            self.base_time.store(soft_time, Ordering::Relaxed);
            self.soft_time.store(soft_time, Ordering::Relaxed);
            self.hard_time.store(hard_time, Ordering::Relaxed);
        } else {
            let (time, inc) = (time[stm].saturating_sub(overhead), inc[stm]);
            let hard_time = ((time as f64 / (W::hard_time_div() as f64 / 4096.0)) as u64
                + inc * W::hard_time_inc() / 4096)
                .min(time);
            let soft_time = ((time as f64 / (W::soft_time_div() as f64 / 4096.0)) as u64
                + inc * W::soft_time_inc() / 4096)
                .min(hard_time);

            self.base_time.store(soft_time, Ordering::Relaxed);
            self.soft_time.store(soft_time, Ordering::Relaxed);
            self.hard_time.store(hard_time, Ordering::Relaxed);
        }

        self.start.store(Instant::now(), Ordering::Relaxed);
    }

    #[inline]
    pub fn set_stop(&self, stop: bool) {
        self.stop.store(stop as u32, Ordering::Relaxed);
        if self.infinite() {
            atomic_wait::wake_all(&self.stop);
        }
    }

    #[inline]
    pub fn wait_for_stop(&self) {
        while !self.should_stop() {
            atomic_wait::wait(&self.stop, 0);
        }
    }

    #[inline]
    pub fn stop_search(&self, thread: &ThreadData) -> bool {
        self.should_stop()
            || thread.nodes.global() >= self.hard_nodes.load(Ordering::Relaxed)
            || (thread.nodes.local().is_multiple_of(1024)
                && thread.id == 0
                && self.check_time.load(Ordering::Relaxed)
                && self.elapsed().as_millis() as u64 > self.hard_time.load(Ordering::Relaxed))
    }

    #[inline]
    pub fn stop_id(&self, depth: u8, nodes: u64) -> bool {
        self.should_stop()
            || depth >= self.depth.load(Ordering::Relaxed)
            || nodes >= self.soft_nodes.load(Ordering::Relaxed)
            || (self.check_time.load(Ordering::Relaxed)
                && self.elapsed().as_millis() as u64 > self.soft_time.load(Ordering::Relaxed))
    }

    #[inline]
    pub fn elapsed(&self) -> Duration {
        self.start.load(Ordering::Relaxed).elapsed()
    }

    #[inline]
    pub fn should_stop(&self) -> bool {
        self.stop.load(Ordering::Relaxed) != 0
    }

    #[inline]
    pub fn infinite(&self) -> bool {
        self.infinite.load(Ordering::Relaxed)
    }
}
