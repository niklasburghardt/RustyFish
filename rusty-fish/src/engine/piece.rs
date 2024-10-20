pub enum Color {
    White = 0,
    Black = 1,
}

pub enum Piece {
    Pawn(Color),
    Knight(Color),
    Bishop(Color),
    Rook(Color),
    Queen(Color),
    King(Color),
}
impl Piece {
    pub fn is_white(&self) -> bool {
        match self {
            self(c) if c == Color::White => true,
            _ => false,
        }
    }
}

pub fn is_white(p: &Piece) -> bool {
    match p {
        p(c) if c == Color::White => true,
        _ => false,
    }
}