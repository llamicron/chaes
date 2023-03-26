use chaes::prelude::*;

fn main() {
    use Square::*;
    let mut bb = BitBoard::default();

    bb.set_piece(Color::Black, PieceKind::Bishop);
    bb.set_squares(vec![A3, E4, H7, B7, H1], true);

    println!("{bb}");
    println!("decimal = {}", bb.bits());
    println!("hex = {:>016x}", bb.bits());
    println!("binary = {:>064b}", bb.bits());
}