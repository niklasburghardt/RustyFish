use std::cmp::PartialEq;

#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq, PartialEq)]
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
#[derive(Eq, PartialEq)]
pub enum Color {
    White = 8,
    Black = 16,
}

impl PartialEq<Color> for &Color {
    fn eq(&self, other: &Color) -> bool {
        match (self, other) {
            (Color::White, Color::White) | (Color::Black, Color::Black) => true,
            _ => false,
        }
    }
}

pub fn is_color(piece: &Piece, color: &Color) -> bool{
    match piece {
        Piece::King(c) | Piece::Queen(c) | Piece::Rook(c) | Piece::Bishop(c) | Piece::Knight(c) | Piece::Pawn(c) => c == *color,
        _ => false,
    }
}





