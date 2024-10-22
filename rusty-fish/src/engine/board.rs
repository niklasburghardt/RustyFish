use std::ptr::copy;
use crate::engine::piece::{Color, Piece};
use crate::engine::piece_move::{PieceMove};
pub struct Board {
    // General information
    pub squares: [Piece; 64],
    pub moving_player: Color,
    pub friendly_color: Color,
    pub opponent_color: Color,
    pub attacked_squares: u64,
    pub ray_squares: u64,
}

impl Board {
    pub fn new() -> Board {
        Board {
            squares: [Piece::None; 64],
            moving_player: Color::White,
            opponent_color: Color::Black,
            friendly_color: Color::White,
            attacked_squares: 0b000,
            ray_squares: 0b000,
        }
    }
    pub fn make_move(&mut self, piece_move: &PieceMove) {
        let piece = self.squares[piece_move.start].clone();
        self.squares[piece_move.start]  = Piece::None;
        self.squares[piece_move.end]    = piece;
    }
}

