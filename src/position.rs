use crate::{bitboard::BitBoard, piece::{Color, PieceKind}};

pub struct Position {
    bitboards: Vec<BitBoard>,
}

impl Position {
    pub fn new() -> Self {
        Self {
            bitboards: Vec::new(),
        }
    }

    /// Will return true if every bitboard in the set
    /// has unique squares active, ie. no two boards claim
    /// the same square
    pub fn no_collisions(&self) -> bool {
        let bits: Vec<&u64> = self.bitboards.iter().map(|bb| bb.bits()).collect();

        let zipped = bits.iter().zip(bits.iter().skip(1));
        for (&a, &b) in zipped {
            if a & b != 0 {
                return false;
            }
        }

        true
    }

    /// Pushes a new bitboard into the Vec
    pub fn add_bitboard(&mut self, new_bb: BitBoard) {
        self.bitboards.push(new_bb)
    }

    pub fn get_bitboard(&self, color: Color, kind: PieceKind) -> Option<&BitBoard> {
        // Go through each board
        for b in &self.bitboards {
            // If it has an associated piece type
            if let Some(assoc_piece) = b.piece() {
                // Check whether it's the piece we're looking for
                if assoc_piece.kind() == &kind && assoc_piece.color() == &color {
                    return Some(b);
                }
            }
            // If there's no associated type, then we move on
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{piece::Color::*, piece::Piece, piece::PieceKind::*, square::Square};

    #[test]
    fn test_position_collision() {
        // Empty positions have no collisions
        let p = Position::new();
        assert!(p.no_collisions());

        // Make 2 bitboards
        let mut bb1 = BitBoard::default();
        let mut bb2 = BitBoard::default();

        // Set the same square active in each
        bb1.set_square(Square::B2, true);
        bb2.set_square(Square::B2, true);

        // Make a new position
        let mut col_pol = Position::new();
        // Add the colliding bitboards
        col_pol.add_bitboard(bb1);
        col_pol.add_bitboard(bb2);

        assert!(!col_pol.no_collisions());
    }

    #[test]
    fn test_get_bitboard_by_piece_type() {
        let mut p = Position::new();
        let bk = Piece::new(Black, King);
        let wk = Piece::new(White, King);
        let bp = Piece::new(Black, Pawn);

        let bb_bk = BitBoard::new(1, Some(bk));
        let bb_wk = BitBoard::new(2, Some(wk));
        let bb_bp = BitBoard::new(4, Some(bp));

        p.add_bitboard(bb_bk);
        p.add_bitboard(bb_wk);
        p.add_bitboard(bb_bp);

        assert!(p.no_collisions());
        let chosen: Option<&BitBoard> = p.get_bitboard(Black, Pawn);
        assert!(chosen.is_some());
        assert!(!chosen.unwrap().is_clear());
        assert_eq!(chosen.unwrap().bits(), &4);
    }
}
