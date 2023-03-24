#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}

impl Piece {
    pub fn to_char(&self) -> char {
        match self {
            Piece::Pawn => '♟',
            Piece::Knight => '♞',
            Piece::Bishop => '♝',
            Piece::Rook => '♜',
            Piece::Queen => '♛',
            Piece::King => '♚',
        }
    }
}