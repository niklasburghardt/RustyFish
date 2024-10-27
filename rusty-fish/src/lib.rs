mod engine;
mod utils;

use std::collections::HashMap;
use std::iter::Map;
use crate::engine::board::Board;
use crate::utils::board_representation;
use crate::engine::piece::Piece;
use crate::engine::piece_move::{Flag, PieceMove, Promotion};
use wasm_bindgen::convert::FromWasmAbi;
use wasm_bindgen::prelude::*;
use crate::engine::piece::Color;
use crate::engine::piece::Color::White;
use crate::engine::position::position_from_fen;

pub fn main () {
    let mut c = ChessEngine::new();
    c.init();
    c.set_board_from_fen("q");
    let mut count = 0;
    for piece in c.board.squares.iter() {
        if *piece != Piece::None {
            count += 1;
        }
    }
    println!("Test successfull, {}", count)
}

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
        self.board.init();
        for i in 0..64 {
            self.board.squares[i] = Piece::None;
        }
    }

    pub fn make_move(&mut self, start: usize, end: usize) {
        self.board.make_move(&PieceMove {start: start as u8, end: end as u8, flag: Flag::None, promotion: Promotion::None});
    }

    pub fn is_white_to_move(&self) -> bool {
        self.board.move_generator.friendly_color == White
    }

    pub fn get_board(&self) -> Vec<String> {
        let mut br = vec!();
        for i in 0..64 {
            br.push(board_representation::piece_to_string(&self.board.squares[i]));
        }
        br
    }

    pub fn generate_moves(&mut self) -> JsValue {
        self.board.generate_moves();
        let mut export: HashMap<u8, Vec<u8>> = HashMap::new();
        let mut export_vec: Vec<Vec<u8>> = vec![];
        for pm in self.board.move_generator.piece_moves.iter() {
            let start = pm.start;
            let end = pm.end;
            if export.contains_key(&start) {
                export.get_mut(&start).unwrap().push(end);
            } else {
                export.insert(start, vec![end]);
            }
        }
        for i in 0..64 {
            if export.contains_key(&i) {
                export_vec.push(export.get(&i).unwrap().clone());
            } else {
                export_vec.push(vec![]);
            }
        }
        JsValue::from_serde(&export_vec).unwrap()
    }

    pub fn set_board_from_fen(&mut self, fen: &str) {
        let pos = position_from_fen(fen);
        self.board.squares = pos.squares;
    }

}

#[wasm_bindgen]
pub fn greet() {
    alert("This comes straight from Rust 2");
}

