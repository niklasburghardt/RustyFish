#[derive(Clone)]
#[derive(Copy)]
pub enum Piece {
    None,
    Pawn(Color),
    Knight(Color),
    Bishop(Color),
    Rook(Color),
    Queen(Color),
    King(Color),
}
#[derive(Clone)]
#[derive(Copy)]
pub enum Color {
    White = 8,
    Black = 16,
}


