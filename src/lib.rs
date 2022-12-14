mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)] // represent each cell as a single byte
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    // Dead variant as 0 and Alive variant as 1 allows us to easily count
    // a cell's live neighbors with addition
    Dead = 0,
    Alive = 1,
}
