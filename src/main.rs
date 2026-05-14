use crate::{
    engine::{Abort, ENGINE_VERSION, Engine},
    util::EPOCH,
};
use colored::Colorize;
use std::{env, io, sync::LazyLock};

pub mod bench;
pub mod board;
pub mod engine;
pub mod position;
pub mod score;
pub mod search;
pub mod types;
pub mod ugi;
pub mod util;

fn main() {
    LazyLock::force(&EPOCH);

    let mut buffer = String::new();
    let mut engine = Engine::new();
    let args = env::args().skip(1).collect::<Vec<String>>();

    if !args.is_empty() {
        for cmd in args {
            if engine.handle(cmd.trim()) == Abort::Yes {
                return;
            }
        }

        return;
    }

    println!("Ristinolla v{} by Tecci", ENGINE_VERSION.bright_green());
    while let Ok(_) = io::stdin().read_line(&mut buffer) {
        if buffer.trim().is_empty() {
            continue;
        }

        if engine.handle(buffer.trim()) == Abort::Yes {
            break;
        }

        buffer.clear();
    }
}
