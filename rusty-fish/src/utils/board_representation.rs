use crate::engine::piece::{Color, Piece};

pub fn x_from_index(index: i8) -> i8 {
    (index % 8)
}

pub fn y_from_index(index: i8) -> i8 {
    7 - (index - x_from_index(index)) / 8
}

pub fn index_from_coords(x: i8, y: i8) -> i8 {
    (7-y) * 8 + x
}

fn abs(val: i8) -> i8 {
    if val < 0 {
       return -val as i8
    }
    val as i8
}
pub fn calculate_distance(i1: i8, i2: i8) -> i8 {
    let xd: i8 = abs( x_from_index(i1) as i8 - x_from_index(i2) as i8);
    let yd: i8 = abs(y_from_index(i1) as i8 - y_from_index(i2) as i8);
    xd + yd


}

pub fn piece_to_string(piece: &Piece) -> String {
    match piece {
        Piece::Pawn(Color::White) => String::from("p-w"),
        Piece::Knight(Color::White) => String::from("n-w"),
        Piece::Bishop(Color::White) => String::from("b-w"),
        Piece::Rook(Color::White) => String::from("r-w"),
        Piece::Queen(Color::White) => String::from("q-w"),
        Piece::King(Color::White) => String::from("k-w"),
        Piece::Pawn(Color::Black) => String::from("p-b"),
        Piece::Knight(Color::Black) => String::from("n-b"),
        Piece::Bishop(Color::Black) => String::from("b-b"),
        Piece::Rook(Color::Black) => String::from("r-b"),
        Piece::Queen(Color::Black) => String::from("q-b"),
        Piece::King(Color::Black) => String::from("k-b"),
        _ => String::from("")
    }
}
