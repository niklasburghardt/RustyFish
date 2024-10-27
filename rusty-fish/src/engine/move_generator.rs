use std::cmp::PartialEq;
use crate::engine::board::Board;
use crate::engine::piece;
use crate::engine::piece::{is_type, Color, Piece};
use crate::engine::piece_move::{Flag, PieceMove, Promotion};
use crate::engine::precomputed::Precomputed;
use crate::utils::board_representation::calculate_distance;

pub struct MoveGenerator {
    pub piece_moves: Vec<PieceMove>,
    is_white_to_move: bool,
    friendly_color: Color,
    opponent_color: Color,
}



impl MoveGenerator {

    pub fn new() -> MoveGenerator {
        MoveGenerator {
            piece_moves: vec![],
            friendly_color: Color::White,
            opponent_color: Color::Black,
            is_white_to_move: true,
        }
    }

    fn add_move_if_legal(&mut self, piece_move: PieceMove) {
        self.piece_moves.push(piece_move);
    }

    fn generate_sliding_piece_moves(&mut self,  squares: &[Piece; 64], precomputed: &Precomputed, index: u8, startDir: u8, endDir: u8) {
        for i in startDir..endDir {
            let direction = precomputed.directional_offset[i as usize];
            let squares_to_edge = precomputed.squares_to_edge[index as usize][i as usize];
            for piece_move in 1..=squares_to_edge {
                let target_square = index as i8 + piece_move*direction;
                if piece::is_color(&squares[target_square as usize], &self.friendly_color) {
                    break;
                }
                self.add_move_if_legal(PieceMove::simple(index, target_square as u8));
                if piece::is_color(&squares[target_square as usize], &self.opponent_color) {
                    break;
                }
            }

        }

    }

    fn generate_sliding_moves(&mut self,  squares: &[Piece; 64], precomputed: &Precomputed) {
        let mut i: u8 = 0;
        for piece in squares.iter() {
            match piece {

                Piece::Rook(c) if c == self.friendly_color => self.generate_sliding_piece_moves(&squares, &precomputed, i, 0, 4),
                Piece::Bishop(c) if c == self.friendly_color => self.generate_sliding_piece_moves(&squares, &precomputed, i, 4, 8),
                Piece::Queen(c) if c == self.friendly_color => self.generate_sliding_piece_moves(&squares, &precomputed, i, 0, 8),
                _ => (),

            }
           i += 1;
        }
    }

    fn generate_king_moves(&mut self, squares: &[Piece; 64], precomputed: &Precomputed) {
        let mut i: i8 = 0;
        for p in squares.iter() {
            if(p == &Piece::King(self.friendly_color)) {
                for dir in precomputed.directional_offset.iter() {
                    let target = i + dir;
                    if target > 0 && target < 64 {
                        if piece::is_color(&squares[target as usize], &self.friendly_color) {
                            continue;
                        }
                        if calculate_distance(i as u8, target as u8) > 2 {
                            continue;
                        }
                        self.add_move_if_legal(PieceMove {start: i as u8, end: target as u8, flag: Flag::KingMove, promotion: Promotion::None});
                    }

                }
            }
            i += 1;
        }
    }
    pub fn generate_legal_moves(&mut self, squares: &[Piece; 64], precomputed: &Precomputed) {
        self.piece_moves.clear();
        self.generate_sliding_moves(squares, precomputed);
        self.generate_king_moves(squares, precomputed);
    }
}