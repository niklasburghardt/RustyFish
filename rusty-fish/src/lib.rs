mod engine;

use wasm_bindgen::prelude::*;
use crate::engine::board::Board;
use crate::engine::piece::{Color, Piece};

#[wasm_bindgen]
pub fn add(left: i32, right: i32) -> i32 {
    left + right + 1
}

#[wasm_bindgen]
pub fn mult(left: i32, right: i32) -> i32 {
    left * right
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

pub fn test_struct() -> bool{
    let b = Board {first: Piece::Pawn(Color::Black)};
    true
}
#[wasm_bindgen]
pub fn greet() {
    alert("This comes straight from Rust 2");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result =add(2, 2);
        assert_eq!(result, 4);
    }
}
