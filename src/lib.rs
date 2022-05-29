use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

// Use 'wee_alloc' as the global allocator
#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen(module = "/www/utils/random.js")]
extern "C" {
    fn random(max: usize) -> usize;
}

#[wasm_bindgen]
#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq)]
pub struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction,
}

impl Snake {
    pub fn new(spawn_index: usize, size: usize) -> Self {
        let mut body = Vec::new();
        for i in 0..size {
            body.push(SnakeCell(spawn_index - i))
        }
        Self {
            body: body,
            direction: Direction::Down,
        }
    }
}

#[wasm_bindgen]
pub struct World {
    width: usize,
    // size: usize,
    reward_cell: usize,
    snake: Snake,
    next_cell: Option<SnakeCell>,
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, snake_index: usize) -> Self {
        let snake = Snake::new(snake_index, 3);
        Self {
            width: width,
            // size: width * width,
            reward_cell: World::gen_reward_cell(width * width, &snake.body),
            snake: snake,
            next_cell: None,
        }
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn snake_head_index(&self) -> usize {
        self.snake.body[0].0
    }
    pub fn change_snake_direction(&mut self, direction: Direction) {
        let next_cell = self.gen_next_snake_cell(&direction);
        if self.snake.body[1].0 == next_cell.0 {
            // can't do a U-turn
            return;
        }
        self.snake.direction = direction;
    }
    fn gen_reward_cell(max: usize, snake_body: &Vec<SnakeCell>) -> usize {
        // reward can't init in snake body
        let mut reward_cell;
        loop {
            reward_cell = random(max);
            if !snake_body.contains(&SnakeCell(reward_cell)) {
                break reward_cell;
            }
        }
    }
    fn gen_next_snake_cell(&self, direction: &Direction) -> SnakeCell {
        let snake_index = self.snake_head_index();
        let row = snake_index / self.width;
        return match direction {
            Direction::Up => {
                let border_hold = snake_index - (self.width - row) * self.width;
                if snake_index == border_hold {
                    SnakeCell(self.width * (self.width - 1) + border_hold)
                } else {
                    SnakeCell(snake_index - self.width)
                }
            }
            Direction::Down => {
                let border_hold = snake_index + (self.width - row) * self.width;
                if snake_index + self.width == border_hold {
                    SnakeCell(border_hold - (row + 1) * self.width)
                } else {
                    SnakeCell(snake_index + self.width)
                }
            }
            Direction::Left => {
                let border_hold = row * self.width;
                if snake_index == border_hold {
                    SnakeCell(border_hold + self.width - 1)
                } else {
                    SnakeCell(snake_index - 1)
                }
            }
            Direction::Right => {
                let border_hold = (row + 1) * self.width - 1;
                if snake_index == border_hold {
                    SnakeCell(border_hold - self.width + 1)
                } else {
                    SnakeCell(snake_index + 1)
                }
            }
        };
    }
    pub fn reward_cell(&self) -> usize {
        self.reward_cell
    }
    pub fn snake_cells(&self) -> *const SnakeCell {
        self.snake.body.as_ptr()
    }
    pub fn snake_length(&self) -> usize {
        self.snake.body.len()
    }
    pub fn update(&mut self) {
        let temp = self.snake.body.clone();
        match self.next_cell {
            Some(cell) => {
                self.snake.body[0] = cell;
                self.next_cell = None;
            }
            None => {
                self.snake.body[0] = self.gen_next_snake_cell(&self.snake.direction);
            }
        }
        let len = self.snake.body.len();
        for i in 1..len {
            self.snake.body[i] = SnakeCell(temp[i - 1].0);
        }

        if self.reward_cell == self.snake_head_index() {
            if self.snake_length() < self.width * self.width {
                self.reward_cell = World::gen_reward_cell(self.width * self.width, &self.snake.body);
            } else {
                self.reward_cell = 123456789;
            }
            self.snake.body.push(SnakeCell(self.snake.body[1].0))
        }
    }
}
