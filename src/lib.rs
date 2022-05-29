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
    alert(name);
}

struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,
}

impl Snake {
    pub fn new(spawn_index: usize) -> Self {
        Self {
            body: vec![SnakeCell(spawn_index)],
        }
    }
}

#[wasm_bindgen]
pub struct World {
    width: usize,
    size: usize,
    snake: Snake,
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize) -> Self {
        Self {
            width: width,
            size: width * width,
            snake: Snake::new(width / 2),
        }
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn snake_head_index(&self) -> usize {
        self.snake.body[0].0
    }
    pub fn update(&mut self) {
        let snake_head_index = self.snake_head_index();
        self.snake.body[0].0 = (snake_head_index + 1) % self.size;
    }
}
