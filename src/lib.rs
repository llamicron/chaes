#![allow(dead_code, unused_imports)]
use log::*;

mod bitboard;
mod square;
mod piece;
mod position;

pub mod prelude {
    pub use super::bitboard::BitBoard;
    pub use super::square::Square;
    pub use super::piece::{Color, Piece, PieceKind};
    pub use super::position::Position;
}
