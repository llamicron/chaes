#![allow(dead_code, unused_imports)]
use log::*;

use chaes::*;

fn init_logging() {
    env_logger::builder().filter_level(LevelFilter::Info).init();
}

fn main() {
    init_logging();

}
