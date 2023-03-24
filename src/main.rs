#![allow(dead_code, unused_imports)]
use log::*;

use crate::{square::Square, piece::Piece};

mod board;
mod square;
mod piece;

fn init_logging() {
    env_logger::builder().filter_level(LevelFilter::Info).init();
}

fn main() {
    init_logging();

}
