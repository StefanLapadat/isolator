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
    let plan = generating_plan::create_plan(&generating_plan::create_request());
    serde_json::to_string(&plan).unwrap()
}

#[wasm_bindgen]
pub fn get_building_triangulized() -> String {
    let triangulized_building = generating_plan::create_building_triangulized();
    serde_json::to_string(&triangulized_building).unwrap()
}

#[wasm_bindgen]
pub fn get_building_levels() -> String {
    let triangulized_building = generating_plan::create_building_levels();
    serde_json::to_string(&triangulized_building).unwrap()
}

#[wasm_bindgen]
pub fn get_building_polygon_walls() -> String {
    let polygon_walls_building = generating_plan::create_building_polygon_walls();
    serde_json::to_string(&polygon_walls_building).unwrap()
}

#[wasm_bindgen]
pub fn get_request() -> String {
    let req = generating_plan::create_request();
    serde_json::to_string(&req).unwrap()
}
