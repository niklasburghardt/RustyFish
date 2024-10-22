mod engine;
mod utils;
use crate::engine::board::Board;
use crate::utils::board_representation;
use crate::engine::piece::Piece;
use crate::engine::piece_move::{Flag, PieceMove, Promotion};
use wasm_bindgen::convert::FromWasmAbi;
use wasm_bindgen::prelude::*;
use crate::engine::piece::Color::Black;

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
        self.board.squares[2] = Piece::Queen(Black);
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
}

#[wasm_bindgen]
pub fn greet() {
    alert("This comes straight from Rust 2");
}
