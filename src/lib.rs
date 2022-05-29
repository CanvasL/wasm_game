use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

// Use 'wee_alloc' as the global allocator
#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

// #[wasm_bindgen]
// extern "C" {
//     pub fn alert(s: &str);
// }

// #[wasm_bindgen]
// pub fn hello(name: &str) {
//     alert(name);
// }

#[wasm_bindgen]
#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction,
}

impl Snake {
    pub fn new(spawn_index: usize) -> Self {
        Self {
            body: vec![SnakeCell(spawn_index)],
            direction: Direction::Down,
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
    pub fn new(width: usize, snake_index: usize) -> Self {
        Self {
            width: width,
            size: width * width,
            snake: Snake::new(snake_index),
        }
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn snake_head_index(&self) -> usize {
        self.snake.body[0].0
    }
    pub fn change_snake_direction(&mut self, direction: Direction) {
        self.snake.direction = direction;
    }
    pub fn update(&mut self) {
        let snake_head_index = self.snake_head_index();
        // self.snake.body[0].0 = (snake_head_index + 1) % self.size;
        // let snake_index = self.set_snake_head(snake_head_index);
        let (row_old, col_old) = self.index_to_cell(snake_head_index);
        let (row, col) = match self.snake.direction {
            Direction::Up => ((row_old - 1) % self.width, col_old),
            Direction::Down => ((row_old + 1) % self.width, col_old),
            Direction::Left => (row_old, (col_old - 1) % self.width),
            Direction::Right => (row_old, (col_old + 1) % self.width),
        };
        let next_index = self.cell_to_index(row, col);
        self.set_snake_head(next_index);
    }
    fn set_snake_head(&mut self, index: usize) {
        self.snake.body[0].0 = index;
    }
    fn index_to_cell(&self, index: usize) -> (usize, usize) {
        (index / self.width, index % self.width)
    }
    fn cell_to_index(&self, row: usize, col: usize) -> usize {
        row * self.width + col
    }
}
