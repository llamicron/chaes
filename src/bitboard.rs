use crate::{piece::*, square::Square};

/// A BitBoard is a way to represent a chess board. It contains a 64-bit integer,
/// one bit for each square. Flipping these bits can denote something about that square.
///
/// For example, we could make a BitBoard with the associated White Bishop piece. Each 1 bit in the
/// 64-bits corresponds with where a white bishop is on the board.
///
/// BitBoards can be used for more than just representing where pieces are. For example, you could create
/// a bitboard that shows all the squares that a piece has vision.
///
/// In this crate, i'll say that a "bit" or "square" is "active" if the bit corresponding with that
/// square is a 1. See [Square](crate::square::Square) for which bits are which squares and how to
/// translate them.
pub struct BitBoard {
    bits: u64,
    piece: Option<Piece>,
}

impl BitBoard {
    /// Creates a new empty board, maybe with an associated piece
    pub fn new(bits: u64, piece: Option<Piece>) -> Self {
        Self { bits, piece }
    }

    pub fn with_piece(color: Color, kind: PieceKind) -> Self {
        let mut bb = BitBoard::default();
        bb.set_piece(color, kind);
        bb
    }

    /// Returns true if all the bits are 0
    pub fn is_clear(&self) -> bool {
        self.bits == 0
    }

    /// Returns the associated piece, if any
    pub fn piece(&self) -> Option<&Piece> {
        self.piece.as_ref()
    }

    /// Sets the associated piece
    pub fn set_piece(&mut self, color: Color, kind: PieceKind) {
        self.piece = Some(Piece::new(color, kind))
    }

    pub fn get_square(&self, sq: Square) -> bool {
        let mask = 1 << sq as u8;
        (self.bits & mask) != 0
    }

    pub fn set_square(&mut self, sq: Square, state: bool) {
        let mask: u64 = 1 << sq as u8;

        if state {
            self.bits |= mask;
        } else {
            self.bits &= !mask;
        }
    }
}


impl Default for BitBoard {
    fn default() -> Self {
        Self::new(0, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_bitboard() {
        let bb = BitBoard::default();
        assert!(bb.is_clear());
        assert_eq!(bb.piece(), None);
    }

    #[test]
    fn test_set_piece() {
        let mut bb = BitBoard::default();
        bb.set_piece(Color::Black, PieceKind::Pawn);
        assert!(bb.is_clear());
        assert!(bb.piece().is_some());
        assert_eq!(bb.piece().unwrap().kind(), &PieceKind::Pawn);
    }

    #[test]
    fn test_set_and_get_square() {
        let mut bb = BitBoard::with_piece(Color::Black, PieceKind::Pawn);
        assert_eq!(bb.get_square(Square::E5), false);
        bb.set_square(Square::E5, true);
        assert_eq!(bb.get_square(Square::E5), true);
        assert_eq!(bb.get_square(Square::E4), false);
        assert_eq!(bb.get_square(Square::E6), false);
        assert!(!bb.is_clear());
    }

    #[test]
    fn test_set_when_already_set() {
        let mut bb = BitBoard::with_piece(Color::Black, PieceKind::Pawn);
        assert_eq!(bb.get_square(Square::E5), false);
        bb.set_square(Square::E5, true);
        assert_eq!(bb.get_square(Square::E5), true);
        bb.set_square(Square::E5, true);
        assert_eq!(bb.get_square(Square::E5), true);
        assert!(!bb.is_clear());
    }

    #[test]
    fn test_set_and_set_back() {
        let mut bb = BitBoard::with_piece(Color::Black, PieceKind::Pawn);
        assert_eq!(bb.get_square(Square::E5), false);
        bb.set_square(Square::E5, true);
        assert_eq!(bb.get_square(Square::E5), true);
        bb.set_square(Square::E5, false);
        assert_eq!(bb.get_square(Square::E5), false);
        assert!(bb.is_clear());
    }
}
