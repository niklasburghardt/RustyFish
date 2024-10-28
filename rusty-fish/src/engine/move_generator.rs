use std::cmp::PartialEq;
use crate::engine::board::Board;
use crate::engine::piece;
use crate::engine::piece::{is_color, is_type, Color, Piece};
use crate::engine::piece_move::{Flag, PieceMove, Promotion};
use crate::engine::precomputed::Precomputed;
use crate::utils::board_representation::{calculate_distance, x_from_index, y_from_index};

pub struct MoveGenerator {
    pub piece_moves: Vec<PieceMove>,
    pub is_white_to_move: bool,
    pub friendly_color: Color,
    pub opponent_color: Color,
    pub epFile: u8,
}



impl MoveGenerator {

    pub fn new() -> MoveGenerator {
        MoveGenerator {
            piece_moves: vec![],
            friendly_color: Color::White,
            opponent_color: Color::Black,
            is_white_to_move: true,
            epFile: 8,
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
                break;
            }
            i += 1;
        }
    }

    fn generate_knight_moves(&mut self, squares: &[Piece; 64], precomputed: &Precomputed) {
        for i in 0..64 {
            if squares[i] != Piece::Knight(self.friendly_color) {
                continue;
            }
            for target in precomputed.knight_moves[i].iter(){
                if *target == -1 {
                    continue
                }
                if is_color(&squares[*target as usize], &self.friendly_color) {
                    continue;
                }
                self.add_move_if_legal(PieceMove{start: i as u8, end: *target as u8, flag: Flag::None, promotion: Promotion::None});



        }
    }
        }

    fn generate_pawn_moves(&mut self, squares: &[Piece; 64], precomputed: &Precomputed) {
        let pre: i8 = match self.is_white_to_move {
            true => 1,
            false => -1,
        };
        for i in 0..64 {
            if squares[i] != Piece::Pawn(self.friendly_color) {
                continue;
            }

            if squares[i+(8*pre) as usize] == Piece::None {
                self.add_move_if_legal(PieceMove{start: i as u8, end: (i as i8 +(8*pre)) as u8, flag: Flag::None, promotion: Promotion::None});
            }
            if x_from_index(i as u8) != 0 && is_color(&squares[i+(7*pre) as usize], &self.opponent_color) {
                self.add_move_if_legal(PieceMove{start: i as u8, end: (i as i8 +7*pre) as u8, flag: Flag::None, promotion: Promotion::None});
            }
            if x_from_index(i as u8) != 7 && is_color(&squares[i+(9*pre) as usize], &self.opponent_color) {
                self.add_move_if_legal(PieceMove {start: i as u8, end: (i as i8 +9*pre) as u8, flag: Flag::None, promotion: Promotion::None});
            }
            let start_file = match self.is_white_to_move {
                true => 6,
                false => 1,
            };
            if y_from_index(i as u8) == start_file && squares[i+(16*pre) as usize] == Piece::None && squares[i+(8*pre) as usize] == Piece::None {
                self.add_move_if_legal(PieceMove{start: i as u8, end: (i as i8+16*pre) as u8, flag: Flag::DoublePawnPush, promotion: Promotion::None});
            }

        }
    }
    pub fn generate_legal_moves(&mut self, squares: &[Piece; 64], precomputed: &Precomputed) {
        self.piece_moves.clear();
        self.generate_sliding_moves(squares, precomputed);
        self.generate_king_moves(squares, precomputed);
        self.generate_knight_moves(squares, precomputed);
        self.generate_pawn_moves(squares, precomputed);
    }

    pub fn switch_players(&mut self) {
        let fc = self.friendly_color;
        self.friendly_color = self.opponent_color;
        self.opponent_color = fc;
        self.is_white_to_move = self.friendly_color == Color::White;
    }
}