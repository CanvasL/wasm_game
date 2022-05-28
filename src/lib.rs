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

#[wasm_bindgen]
struct World {
    width: usize,
    size: usize,
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize) -> Self {
        Self {
            width: width,
            size: width * width,
        }
    }
    pub fn width(&self) -> usize {
        self.width
    }
}
