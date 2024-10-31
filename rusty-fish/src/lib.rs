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


// For testing and debugging purposes
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

// using alert in browser
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}
// This is the engine itself that is going to be accessed from the browser
#[wasm_bindgen]
struct ChessEngine {
    board: Board,
}

// These are all the functions that the end user (the browser)
// Has access on
#[wasm_bindgen]
impl ChessEngine {

    // Constructor for Chess Engine
    pub fn new() -> ChessEngine {
        ChessEngine {board: Board::new()}
    }

    // Initialize the board and the game state
    // This should only be called when no old data is needed anymore
    pub fn init(&mut self) {
        self.board.init();
        for i in 0..64 {
            self.board.squares[i] = Piece::None;
        }
    }

    // make a move TODO: if legal
    pub fn make_move(&mut self, start: usize, end: usize) {
        self.board.make_move(PieceMove {start: start as i8, end: end as i8, flag: Flag::None, promotion: Promotion::None});
    }

    // Check if white is the moving player
    // WHITE = TRUE
    // BLACK = FALSE
    pub fn is_white_to_move(&self) -> bool {
        self.board.move_generator.borrow().friendly_color == White
    }

    // returns the board in a state that can be read by our frontend
    pub fn get_board(&self) -> Vec<String> {
        let mut br = vec!();
        for i in 0..64 {
            br.push(board_representation::piece_to_string(&self.board.squares[i]));
        }
        br
    }

    // generates all the legal moves and returns a format the browser understands
    pub fn generate_moves(&mut self) -> JsValue {
        self.board.generate_moves();
        let mut export: HashMap<i8, Vec<i8>> = HashMap::new();
        let mut export_vec: Vec<Vec<i8>> = vec![];
        for pm in self.board.move_generator.borrow().piece_moves.iter() {
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

    // sets the board from a given fen string
    pub fn set_board_from_fen(&mut self, fen: &str) {
        let pos = position_from_fen(fen);
        self.board.squares = pos.squares;
    }

}

#[wasm_bindgen]
pub fn greet() {
    alert("This comes straight from Rust 2");
}

