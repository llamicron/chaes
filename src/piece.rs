#[derive(Debug, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Piece {
    color: Color,
    kind: PieceKind,
}

impl Piece {
    pub fn new(color: Color, kind: PieceKind) -> Self {
        Self { color, kind }
    }

    pub fn is_white(&self) -> bool {
        self.color == Color::White
    }

    pub fn color(&self) -> &Color {
        &self.color
    }

    pub fn kind(&self) -> &PieceKind {
        &self.kind
    }
}

impl Piece {
    pub fn to_char(&self) -> char {
        match (&self.kind, self.is_white()) {
            (PieceKind::Pawn, false) => '♟',
            (PieceKind::Knight, false) => '♞',
            (PieceKind::Bishop, false) => '♝',
            (PieceKind::Rook, false) => '♜',
            (PieceKind::Queen, false) => '♛',
            (PieceKind::King, false) => '♚',
            (PieceKind::Pawn, true) => '♙',
            (PieceKind::Knight, true) => '♘',
            (PieceKind::Bishop, true) => '♗',
            (PieceKind::Rook, true) => '♖',
            (PieceKind::Queen, true) => '♕',
            (PieceKind::King, true) => '♔',
        }
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_piece() {
        let wb = Piece::new(Color::White, PieceKind::Bishop);
        let bb = Piece::new(Color::Black, PieceKind::Bishop);
        assert!(wb.is_white());
        assert!(!bb.is_white());
        assert_eq!(wb.kind(), &PieceKind::Bishop);
        assert_eq!(bb.kind(), &PieceKind::Bishop);
    }
}