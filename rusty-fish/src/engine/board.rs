use std::cell::RefCell;
use std::cmp::PartialEq;
use std::ptr::copy;
use crate::engine::move_generator::MoveGenerator;
use crate::engine::piece::{is_color, is_piece, is_type, Color, Piece};
use crate::engine::piece::Color::White;
use crate::engine::piece_move::{Flag, PieceMove, Promotion};
use crate::engine::position::position_from_fen;
use crate::engine::precomputed::Precomputed;


// indicies that cause lose of castle right
const WHITE_KC_INDEX: i8 = 7;
const WHITE_QC_INDEX: i8 = 0;
const BLACK_KC_INDEX: i8 = 63;
const BLACK_QC_INDEX: i8 = 56;
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
    pub epIndex: i8,
    pub last_move: PieceMove,
    pub move_generator: RefCell<MoveGenerator>,
    pub precomputed: Precomputed,
    //0-3 casteling legality
    //4-7 file of ep square (starting at 1 | 0 is no ep)
    //8-13 captured piece
    //14- ...fifty move counter
    pub game_state: u32,
    pub game_stack: Vec<u32>,
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
            move_generator: RefCell::from(MoveGenerator::new()),
            precomputed: Precomputed::new(),
            epIndex: -1,
            last_move: PieceMove::none(),
            game_state: 0,
            game_stack: vec![],
        }
    }

    pub fn init(&mut self) {
        self.precomputed.init();
    }


    pub fn load_position_from_fen(&mut self, fen: &str) {
        let pos = position_from_fen(fen);
        self.squares = pos.squares;
    }

    pub fn switch_players(&mut self) {
        self.move_generator.borrow_mut().switch_players();

    }

    pub fn generate_moves(&self) {

        self.move_generator.borrow_mut().generate_legal_moves(self);
    }





    pub fn make_move(&mut self, piece_move: &PieceMove) {
        self.last_move = piece_move.clone();

        let start_index = piece_move.start;
        let end_index = piece_move.end;
        let flag = piece_move.flag.clone();
        let move_piece = self.squares[start_index as usize];
        let cap_piece = self.squares[end_index as usize];
        if is_piece(&cap_piece) && flag != Flag::EP {
            self.squares[end_index as usize] = Piece::None;
        }
        self.squares[start_index as usize] = Piece::None;
        self.squares[end_index as usize] = move_piece.clone();
        if flag == Flag::DoublePawnPush {
            self.epIndex = end_index as i8;
        } else {
            self.epIndex = -1;
        }
        if flag == Flag::EP {

            let enemy_pawn_index: i8 = end_index + if self.friendly_color == Color::White  {-8}  else {8};
            self.squares[enemy_pawn_index as usize] = Piece::None;
        }
        let black_offset: i8 = if self.friendly_color == Color::White {0} else {56};
        if flag == Flag::CastleKing {
            self.squares[(WHITE_KC_INDEX + black_offset) as usize] = Piece::None;
            self.squares[(WHITE_KC_INDEX + black_offset - 2) as usize] = Piece::Rook(self.friendly_color);
        }
        if flag == Flag::CastleQueen {
            self.squares[(WHITE_QC_INDEX+black_offset) as usize] = Piece::None;
            self.squares[(WHITE_QC_INDEX+black_offset+3) as usize] = Piece::Rook(self.friendly_color)
        }

        if flag == Flag::CastleQueen || flag == Flag::CastleKing || flag == Flag::KingMove {
            if self.friendly_color == White {
                self.whiteKingCastle = false;
                self.whiteQueenCastle = false;
            } else {
                self.blackKingCastle = false;
                self.blackQueenCastle = false;
            }

        }
        if flag == Flag::CastleKing || flag == Flag::CastleQueen {
            if start_index == WHITE_KC_INDEX || end_index == WHITE_KC_INDEX {
                self.whiteKingCastle = false;
            } else if start_index == WHITE_QC_INDEX || end_index == WHITE_QC_INDEX {
                self.whiteQueenCastle = false;
            } else if start_index == BLACK_KC_INDEX || end_index == BLACK_KC_INDEX {
                self.blackKingCastle = false;
            }
             else if start_index == BLACK_QC_INDEX || end_index == BLACK_QC_INDEX {
                 self.blackQueenCastle = false;
             }
        }

        if flag == Flag::Promotion {
            match piece_move.promotion {
              Promotion::Queen => self.squares[end_index as usize] = Piece::Queen(self.friendly_color),
                _ => (),
            }
        }
        self.switch_players();
        self.generate_moves();
    }

    pub fn unmake_move(&mut self, piece_move: &PieceMove) {
        self.switch_players();
        let undoing_white = self.friendly_color == White;
        let moved_from = piece_move.start;
        let moved_to = piece_move.end;
        let flag = piece_move.flag.clone();

        let moved_piece = self.squares[moved_to as usize];


    }


}

