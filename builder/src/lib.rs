mod utils;

use wasm_bindgen::prelude::*;
use generating_plan;

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
pub fn get_plan() -> String {
    let plan = generating_plan::create_plan();
    serde_json::to_string(&plan).unwrap()
}
