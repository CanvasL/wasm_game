use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

// Use 'wee_alloc' as the global allocator
#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn hello(name: &str) {
    unsafe {
        alert(name);
    }
}