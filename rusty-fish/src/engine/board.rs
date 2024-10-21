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
    pub fn make_move(&self, piece_move: &PieceMove) {

    }
}