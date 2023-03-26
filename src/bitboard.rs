use std::fmt;

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

    /// Lets you borrow the bits
    pub fn bits(&self) -> &u64 {
        &self.bits
    }

    /// Creates a new, empty bitboard with an associated piece
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

    /// Gets the status of a square: active or not
    pub fn get_square(&self, sq: Square) -> bool {
        let mask = 1 << sq as u8;
        (self.bits & mask) != 0
    }

    /// Sets the status of a square
    pub fn set_square(&mut self, sq: Square, state: bool) {
        let mask: u64 = 1 << sq as u8;

        if state {
            self.bits |= mask;
        } else {
            self.bits &= !mask;
        }
    }

    /// Sets multiple squares at once
    pub fn set_squares(&mut self, sqs: Vec<Square>, state: bool) {
        for sq in sqs {
            self.set_square(sq, state)
        }
    }

    pub fn potential_moves(&self) -> u64 {
        todo!();
    }
}

impl fmt::Display for BitBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let piece_char: char = match self.piece() {
            Some(piece) => piece.to_char(),
            None => '1'
        };



        write!(f, "   A  B  C  D  E  F  G  H ").expect("Couldn't write to formatter");
        let mut rank = 8;
        for i in 0..64 {

            if i % 8 == 0 {
                write!(f, "\n{rank} ").expect("Couldn't write to formatter");
                rank -= 1;
            }
            let mask = 1 << i;
            if self.bits & mask != 0 {
                write!(f, " {piece_char} ").expect("Couldn't write to formatter");
            } else {
                write!(f, " 0 ").expect("Couldn't write to formatter");
            }
        }
        writeln!(f, "\n   A  B  C  D  E  F  G  H ")
    }
}

impl Default for BitBoard {
    /// A default, empty board with no associated piece
    fn default() -> Self {
        Self::new(0, None)
    }
}

impl Into<Vec<Square>> for BitBoard {
    fn into(self) -> Vec<Square> {
        let mut squares = Vec::new();
        for i in 0..64 {
            if self.bits & (1 << i) != 0 {
                squares.push(Square::try_from(i).unwrap())
            }
        }
        squares
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Square::*;

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

    #[test]
    fn test_set_squares() {
        let mut bb = BitBoard::default();
        assert!(bb.is_clear());
        bb.set_squares(vec![A1, H7, E7, H3, D2], true);
        assert!(bb.get_square(A1));
        assert!(bb.get_square(H7));
        assert!(bb.get_square(E7));
        assert!(bb.get_square(H3));
        assert!(bb.get_square(D2));
    }

    #[test]
    fn test_bitboard_into_vec_of_squares() {
        let mut bb = BitBoard::default();

        bb.set_squares(vec![H7, E3, A2], true);
        
        let found: Vec<Square> = bb.into();
        assert_eq!(found, vec![H7, E3, A2]);
    }

    
}
