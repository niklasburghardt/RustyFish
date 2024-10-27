use std::ptr::copy;
use crate::engine::move_generator::MoveGenerator;
use crate::engine::piece::{Color, Piece};
use crate::engine::piece_move::{PieceMove};
use crate::engine::position::position_from_fen;
use crate::engine::precomputed::Precomputed;

pub struct Board {
    // General information
    pub squares: [Piece; 64],
    pub friendly_color: Color,
    pub opponent_color: Color,
    pub attacked_squares: u64,
    pub ray_squares: u64,
    pub whiteKingCastle: bool,
    pub whiteQueenCastle: bool,
    pub blackKingCastle: bool,
    pub blackQueenCastle: bool,
    pub move_generator: MoveGenerator,
    pub precomputed: Precomputed,
}

impl Board {
    pub fn new() -> Board {
        Board {
            squares: [Piece::None; 64],
            opponent_color: Color::Black,
            friendly_color: Color::White,
            attacked_squares: 0b000,
            ray_squares: 0b000,
            whiteKingCastle: false,
            whiteQueenCastle: false,
            blackKingCastle: false,
            blackQueenCastle: false,
            move_generator: MoveGenerator::new(),
            precomputed: Precomputed::new(),
        }
    }

    pub fn init(&mut self) {
        self.precomputed.init();
    }


    pub fn load_positon_from_fen(&mut self, fen: &str) {
        let pos = position_from_fen(fen);
        self.squares = pos.squares;
    }

    pub fn switch_players(&mut self) {
        self.move_generator.switch_players();

    }

    pub fn generate_moves(&mut self) {
        self.move_generator.generate_legal_moves(&self.squares, &self.precomputed);
    }





    pub fn make_move(&mut self, piece_move: &PieceMove) {
        let piece = self.squares[piece_move.start as usize].clone();
        self.squares[piece_move.start as usize]  = Piece::None;
        self.squares[piece_move.end as usize]    = piece;
        self.switch_players();
    }


}

