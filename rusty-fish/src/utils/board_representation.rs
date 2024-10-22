use crate::engine::piece::{Color, Piece};

pub fn x_from_index(index: u8) -> u8 {
    (index % 8)
}

pub fn y_from_index(index: u8) -> u8 {
    7 - (index - x_from_index(index)) / 8
}

pub fn index_from_coords(x: u8, y: u8) -> u8 {
    (7-y) * 8 + x
}

pub fn piece_to_string(piece: &Piece) -> String {
    match piece {
        Piece::Pawn(Color::White) => String::from("p-w"),
        Piece::Knight(Color::White) => String::from("n-w"),
        Piece::Bishop(Color::White) => String::from("b-w"),
        Piece::Rook(Color::White) => String::from("r-w"),
        Piece::Queen(Color::White) => String::from("q-w"),
        Piece::King(Color::White) => String::from("k-w"),
        Piece::Pawn(Color::Black) => String::from("p-w"),
        Piece::Knight(Color::Black) => String::from("n-w"),
        Piece::Bishop(Color::Black) => String::from("b-w"),
        Piece::Rook(Color::Black) => String::from("r-b"),
        Piece::Queen(Color::Black) => String::from("q-b"),
        Piece::King(Color::Black) => String::from("k-b"),
        _ => String::from("")
    }
}