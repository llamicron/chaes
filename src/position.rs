use crate::bitboard::BitBoard;

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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::square::Square;

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
}
