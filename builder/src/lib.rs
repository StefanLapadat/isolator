mod utils;

use wasm_bindgen::prelude::*;
use generating_plan;
use request_generation;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: String);
}

#[wasm_bindgen]
pub fn get_plan(request_id: i32, tile_length: f64, tile_height: f64, tile_width: f64) -> String {
    let plan = generating_plan::create_plan(
        &request_generation::create_request(request_id, tile_length, tile_height, tile_width), alert);
    serde_json::to_string(&plan).unwrap()
}
