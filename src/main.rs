#![allow(dead_code, unused_imports)]
use log::*;

use crate::square::Square;

mod board;
mod square;
mod piece;

fn init_logging() {
    env_logger::builder().filter_level(LevelFilter::Info).init();
}

fn main() {
    init_logging();

    // let mut board = board::Board::new();
    // board.set_bishops(0x0102030405060708);

    // for i in (0..64).rev() {
    //     if board.bishops() & (1 << i) != 0 {
    //         print!("1");
    //     } else {
    //         print!("0");
    //     }
    // }
    // println!();

    // // let bytes = format!("{:064b}", board.bishops());
    // // for i in 1..=64 {
    // //     print!(" {} ", bytes.chars().nth(i-1).unwrap());
    // //     if i % 8 == 0 {
    // //         println!()
    // //     }
    // // }
    // println!("{:064b}", board.bishops());
}
