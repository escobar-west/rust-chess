#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unreachable_code)]
mod board;
mod gamestate;
mod pieces;

pub use gamestate::GameState;
use tikv_jemallocator::Jemalloc;

#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;
