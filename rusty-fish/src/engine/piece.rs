pub enum Color {
    White = 0,
    Black = 1,
}

pub enum Figure {
    Pawn(Color),
    Knight(Color),
    Bishop(Color),
    Rook(Color),
    Queen(Color),
    King(Color),
}

struct Piece {
    figure: Figure,
    color: Color,
}
