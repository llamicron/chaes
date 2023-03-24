use std::collections::HashMap;
use log::*;

use crate::{piece::{Piece, self}, square::Square};

pub type BitBoard = std::collections::HashMap<Piece, u64>;

pub trait Board {
    fn new_empty() -> Self;
    fn get_all(&self, piece_type: Piece) -> &u64;
    fn get_single(&self, square: Square) -> Option<&Piece>;
    fn set_all(&mut self, piece_type: Piece, new_positions: u64);
    fn set_single(&mut self, piece: Piece, square: Square);

}

impl Board for BitBoard {
    /// Create a new, empty chess board
    /// Note that you should avoid using `HashMap::new()` (implemented on this type) because
    /// it will not set the Piece keys properly.
    fn new_empty() -> Self {
        let mut bb = HashMap::new();
        bb.insert(Piece::Pawn, 0);
        bb.insert(Piece::Knight, 0);
        bb.insert(Piece::Bishop, 0);
        bb.insert(Piece::Rook, 0);
        bb.insert(Piece::Queen, 0);
        bb.insert(Piece::King, 0);
        println!("{:#?}", bb);
        bb
    }

    /// Get the positions of all pieces of the given type as a u64.
    /// Each bit of the u64 corresponds with a square. See Square type.
    fn get_all(&self, piece_type: Piece) -> &u64 {
        if let Some(positions) = self.get(&piece_type) {
            return positions;
        }
        error!("Piece keys were not set properly. You should use BitBoard::new_empty() instead of BitBoard::new()");
        panic!("Bad Piece keys in board initialization");
    }

    /// Gets the piece type on a single square
    fn get_single(&self, square: Square) -> Option<&Piece> {
        // Start with 1 (000...0001) and shift left until the 1
        // is at the ith bit, where i = the square index.
        // See Square enum for square indices.
        let mask = 1 << square as u8;

        // For all the positions of each piece type
        for (piece_type, positions) in self.iter() {
            // If we & the mask with the positions and get something
            // other than 0, then this piece type is on that square
            if positions & mask != 0 {
                return Some(piece_type);
            }
        }
        None
    }

    // TODO: Implement an error when a piece is already occupying the square

    /// Set the positions of a certain piece type
    fn set_all(&mut self, piece_type: Piece, new_positions: u64) {
        if let Some(positions) = self.get_mut(&piece_type) {
            *positions = new_positions;
        }
    }

    fn set_single(&mut self, piece_type: Piece, square: Square) {
        let mask = 1 << square as u8;

        if let Some(positions) = self.get_mut(&piece_type) {
            *positions = *positions | mask;
        }
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_board() {
        let mut board = BitBoard::new_empty();
        assert_eq!(board.get_all(Piece::Pawn), &0x0000000000000000);
        assert_eq!(board.get_all(Piece::Knight), &0x0000000000000000);
        assert_eq!(board.get_all(Piece::Bishop), &0x0000000000000000);
        assert_eq!(board.get_all(Piece::Rook), &0x0000000000000000);
        assert_eq!(board.get_all(Piece::Queen), &0x0000000000000000);
        assert_eq!(board.get_all(Piece::King), &0x0000000000000000);

        board.set_all(Piece::Knight, 0xFF);
        assert_eq!(board.get_all(Piece::Knight), &0xFF);
    }

    #[test]
    fn test_get_set_single() {
        let mut board = BitBoard::new_empty();

        // Assert that every square is empty
        for i in 0..63 {
            assert_eq!(board.get_single(Square::try_from(i).unwrap()), None);
        }

        board.set_single(Piece::Pawn, Square::E6);
        assert_eq!(board.get_single(Square::E6), Some(&Piece::Pawn));
        assert_eq!(board.get_single(Square::E8), None);
        assert_eq!(board.get_single(Square::F6), None);


    }
}