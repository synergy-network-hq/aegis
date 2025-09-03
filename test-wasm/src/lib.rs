use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn test_function() -> String {
    "Hello from WASM!".to_string()
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
