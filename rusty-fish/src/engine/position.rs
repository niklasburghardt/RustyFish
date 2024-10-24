use std::cmp::PartialEq;
use std::f32::RADIX;
use crate::engine::piece::{Color, Piece};

pub struct Position {
    pub squares: [Piece; 64],
    white_castle_kingside: bool,
    white_castle_queenside: bool,
    black_castle_kingside: bool,
    black_castle_queenside: bool,
    ep_file: i8,
    white_to_move: bool,
    ply_count: u16,
}

impl Position {
    fn new() -> Position {
        Position {
            squares: [Piece::None; 64],
            white_castle_kingside: false,
            white_castle_queenside: false,
            black_castle_kingside: false,
            black_castle_queenside: false,
            ep_file: -1,
            white_to_move: true,
            ply_count: 0
        }
    }

}

pub fn position_from_fen(fen: &str) -> [Piece; 64] {
    let mut new_squares = [Piece::None; 64];
    let mut file = 0;
    let mut rank = 7;

    for symbol in fen.chars() {
        if symbol == '/' {
            // Move to the next rank (row)
            file = 0;
            rank -= 1;
        } else if let Some(digit) = symbol.to_digit(10) {
            // Increment the file position by the number of empty squares
            file += digit;
        } else {
            // Match each piece symbol and set the corresponding square
            let piece = match symbol {
                'P' => Piece::Pawn(Color::White),
                'p' => Piece::Pawn(Color::Black),
                'R' => Piece::Rook(Color::White),
                'r' => Piece::Rook(Color::Black),
                'N' => Piece::Knight(Color::White),
                'n' => Piece::Knight(Color::Black),
                'B' => Piece::Bishop(Color::White),
                'b' => Piece::Bishop(Color::Black),
                'Q' => Piece::Queen(Color::White),
                'q' => Piece::Queen(Color::Black),
                'K' => Piece::King(Color::White),
                'k' => Piece::King(Color::Black),
                _   => Piece::None, // Invalid or unrecognized symbol
            };

            if piece != Piece::None {
               // let index: usize = (rank * 8 + file) as usize;
                new_squares[0] = piece;
                file += 1;
            } else {
                // Handle invalid symbol (optional)
                panic!("Invalid FEN symbol: {}", symbol);
            }
        }
    }

    new_squares
}

