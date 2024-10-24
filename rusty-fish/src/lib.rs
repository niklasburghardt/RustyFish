mod engine;
mod utils;
use crate::engine::board::Board;
use crate::utils::board_representation;
use crate::engine::piece::Piece;
use crate::engine::piece_move::{Flag, PieceMove, Promotion};
use wasm_bindgen::convert::FromWasmAbi;
use wasm_bindgen::prelude::*;
use crate::engine::piece::Color;
use crate::engine::position::position_from_fen;

#[wasm_bindgen]
pub fn add(left: i32, right: i32) -> i32 {
    left + right + 1
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}
#[wasm_bindgen]
struct ChessEngine {
    board: Board,
}
#[wasm_bindgen]
impl ChessEngine {
    pub fn new() -> ChessEngine {
        ChessEngine {board: Board::new()}
    }

    pub fn init(&mut self) {
        self.board.squares[2] = Piece::Pawn(Color::White);

    }

    pub fn make_move(&mut self, start: usize, end: usize) {
        self.board.make_move(&PieceMove {start, end, flag: Flag::None, promotion: Promotion::None});
    }

    pub fn get_board(&self) -> Vec<String> {
        let mut br = vec!();
        for i in 0..64 {
            br.push(board_representation::piece_to_string(&self.board.squares[i]));
        }
        br
    }

    pub fn set_board_from_fen(&mut self, fen: &str) {
        self.start_position();
    }

    pub fn start_position(&mut self) {
        self.board.squares[0] = Piece::Rook(Color::White);
        self.board.squares[1] = Piece::Knight(Color::White);
        self.board.squares[2] = Piece::Bishop(Color::White);
        self.board.squares[3] = Piece::Queen(Color::White);
        self.board.squares[4] = Piece::King(Color::White);
        self.board.squares[5] = Piece::Bishop(Color::White);
        self.board.squares[6] = Piece::Knight(Color::White);
        self.board.squares[7] = Piece::Rook(Color::White);

        self.board.squares[8] = Piece::Pawn(Color::White);
        self.board.squares[9] = Piece::Pawn(Color::White);
        self.board.squares[10] = Piece::Pawn(Color::White);
        self.board.squares[11] = Piece::Pawn(Color::White);
        self.board.squares[12] = Piece::Pawn(Color::White);
        self.board.squares[13] = Piece::Pawn(Color::White);
        self.board.squares[14] = Piece::Pawn(Color::White);
        self.board.squares[15] = Piece::Pawn(Color::White);

        // Empty squares (from index 16 to 47)
        // These squares are initialized to Piece::None
        for i in 16..48 {
            self.board.squares[i] = Piece::None;
        }

        self.board.squares[48] = Piece::Pawn(Color::Black);
        self.board.squares[49] = Piece::Pawn(Color::Black);
        self.board.squares[50] = Piece::Pawn(Color::Black);
        self.board.squares[51] = Piece::Pawn(Color::Black);
        self.board.squares[52] = Piece::Pawn(Color::Black);
        self.board.squares[53] = Piece::Pawn(Color::Black);
        self.board.squares[54] = Piece::Pawn(Color::Black);
        self.board.squares[55] = Piece::Pawn(Color::Black);

        self.board.squares[56] = Piece::Rook(Color::Black);
        self.board.squares[57] = Piece::Knight(Color::Black);
        self.board.squares[58] = Piece::Bishop(Color::Black);
        self.board.squares[59] = Piece::Queen(Color::Black);
        self.board.squares[60] = Piece::King(Color::Black);
        self.board.squares[61] = Piece::Bishop(Color::Black);
        self.board.squares[62] = Piece::Knight(Color::Black);
        self.board.squares[63] = Piece::Rook(Color::Black);


    }
}

#[wasm_bindgen]
pub fn greet() {
    alert("This comes straight from Rust 2");
}

