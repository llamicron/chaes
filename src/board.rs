use crate::{piece::{Piece, self}, square::Square};

pub struct Board {
    pawns: u64,
    knights: u64,
    bishops: u64,
    rooks: u64,
    queens: u64,
    kings: u64
}

impl Board {
    /// Create a new, empty chess board
    pub fn new() -> Self {
        Self {
            pawns: 0,
            knights: 0,
            bishops: 0,
            rooks: 0,
            queens: 0,
            kings: 0
        }
    }

    /// Get the positions of all pieces of the given type as a u64.
    /// Each bit of the u64 corresponds with a square. See Square type.
    pub fn get(&self, piece_type: Piece) -> u64 {
        match piece_type {
            Piece::Pawn => self.pawns,
            Piece::Knight => self.knights,
            Piece::Bishop => self.bishops,
            Piece::Rook => self.rooks,
            Piece::Queen => self.queens,
            Piece::King => self.kings,
        }
    }

    /// Gets the piece type on a singe square
    pub fn get_single(&self, square: Square) -> Option<Piece> {
        let mask = 1 << square as u8;
        
        if self.pawns & mask != 0 {
            return Some(Piece::Pawn);
        }
        if self.knights & mask != 0 {
            return Some(Piece::Knight);
        }
        if self.bishops & mask != 0 {
            return Some(Piece::Bishop);
        }
        if self.rooks & mask != 0 {
            return Some(Piece::Rook);
        }
        if self.queens & mask != 0 {
            return Some(Piece::Queen);
        }
        if self.kings & mask != 0 {
            return Some(Piece::King);
        }
        None
    }

    /// Set the positions of a certain piece type
    pub fn set(&mut self, piece_type: Piece, new_positions: u64) {
        match piece_type {
            Piece::Pawn => self.pawns = new_positions,
            Piece::Knight => self.knights = new_positions,
            Piece::Bishop => self.bishops = new_positions,
            Piece::Rook => self.rooks = new_positions,
            Piece::Queen => self.queens = new_positions,
            Piece::King => self.kings = new_positions,
        }
    }

    pub fn set_single(&mut self, piece: Piece, square: Square) {
        let mask = 1 << square as u8;
        match piece {
            Piece::Pawn => self.pawns = self.pawns | mask,
            Piece::Knight => self.knights = self.knights | mask,
            Piece::Bishop => self.bishops = self.bishops | mask,
            Piece::Rook => self.rooks = self.rooks | mask,
            Piece::Queen => self.queens = self.queens | mask,
            Piece::King => self.kings = self.kings | mask,
        }
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_board() {
        let mut board = Board::new();
        assert_eq!(board.get(Piece::Pawn), 0x0000000000000000);
        assert_eq!(board.get(Piece::Knight), 0x0000000000000000);
        assert_eq!(board.get(Piece::Bishop), 0x0000000000000000);
        assert_eq!(board.get(Piece::Rook), 0x0000000000000000);
        assert_eq!(board.get(Piece::Queen), 0x0000000000000000);
        assert_eq!(board.get(Piece::King), 0x0000000000000000);

        board.set(Piece::Knight, 0xFF);
        assert_eq!(board.get(Piece::Knight), 0xFF);
    }

    #[test]
    fn test_get_set_single() {
        let mut board = Board::new();

        board.set_single(Piece::Pawn, Square::E6);
        assert_eq!(board.get_single(Square::E6), Some(Piece::Pawn));
        assert_eq!(board.get_single(Square::E8), None);
    }
}