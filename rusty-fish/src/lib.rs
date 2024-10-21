mod engine;

use wasm_bindgen::prelude::*;
use crate::engine::board::Board;

#[wasm_bindgen]
pub fn add(left: i32, right: i32) -> i32 {
    left + right + 1
}


#[wasm_bindgen]
extern {
    fn alert(s: &str);
}


#[wasm_bindgen]
pub fn greet() {
    alert("This comes straight from Rust 2");
}
