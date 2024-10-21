pub enum Piece {
    None,
    Pawn(Color),
    Knight(Color),
    Bishop(Color),
    Rook(Color),
    Queen(Color),
    King(Color),
}

pub enum Color {
    White = 8,
    Black = 16,
}

pub fn is_color(piece: &Piece, color: Color) -> bool {
    let p = Piece::Pawn(Color::Black);
    match piece {
        piece(c) if c == color => true,
        _ => false,
    }
}
