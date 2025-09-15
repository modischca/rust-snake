use rusqlite::Error;

use crate::db;
use crate::game::types::GameStatus;
use crate::game::{Cell, Direction};
pub const BOARD_COLS: usize = 16;
pub const BOARD_ROWS: usize = 16;
use std::time::SystemTime;

pub struct Game {
    pub score: u16,
    pub next_food_target: Option<Pos>,
    pub board: [[Cell; BOARD_COLS]; BOARD_ROWS],
    pub game_start_at: std::time::SystemTime,
    pub snake: Snake,
    pub db_id: Option<u16>,
    pub game_status: GameStatus,
    pub player_name: String,
}

impl Game {
    pub fn new() -> Self {
        let start_direction = Direction::RIGHT;
        let start_pos = Pos { x: 5, y: 5 };
        let mut parts: Vec<Pos> = vec![start_pos];
        let size = 6;
        for _i in 0..size {
            let pos = parts[_i].next(&start_direction);
            parts.push(pos);
        }
        let snake = Snake {
            direction: Direction::RIGHT,
            parts_x_y: parts,
        };
        Self {
            score: 0,
            next_food_target: None,
            board: [[Cell::EMPTY; BOARD_COLS]; BOARD_ROWS],
            game_start_at: SystemTime::now(),
            snake: snake,
            db_id: None,
            game_status: GameStatus::RUNNING,
            player_name: "Morten".to_string(),
        }
    }

    pub fn save(&mut self) {
        if self.db_id.is_none() {
            db::insert(self);
        } else {
            println!("Saving game....");
        }
    }

    pub fn update_board(&mut self) {
        // The new grid
        if (self.next_food_target.is_some()) {
            let a = self.snake.parts_x_y.iter().find(|p| {
                p.x == self.next_food_target.unwrap().x && p.y == self.next_food_target.unwrap().y
            });
            if a.is_some() {
                self.score = self.score + 10;
                self.next_food_target = None;
            }
        }

        let mut grid: [[Cell; BOARD_COLS]; BOARD_ROWS] = [[Cell::EMPTY; BOARD_COLS]; BOARD_ROWS];

        for pos in &self.snake.parts_x_y {
            grid[pos.y as usize][pos.x as usize] = Cell::SNAKE_BODY;
        }
        let head = &self.snake.parts_x_y.iter().last().unwrap();
        grid[head.y as usize][head.x as usize] = Cell::SNAKE_HEAD;

        if self.next_food_target.is_none() {
            // Add food
            let x = rand::random_range(0..BOARD_ROWS - 1);
            let y = rand::random_range(0..BOARD_COLS - 1);
            let food_at_pos = Pos {
                x: x as u16,
                y: y as u16,
            };
            self.next_food_target = Some(food_at_pos);
            grid[y][x] = Cell::FOOD;
        } else {
            grid[self.next_food_target.unwrap().y as usize]
                [self.next_food_target.unwrap().x as usize] = Cell::FOOD;
        }

        self.board = grid;
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Pos {
    pub x: u16,
    pub y: u16,
}

impl Pos {
    // Returns a new Pos with the next coordinates.
    pub fn next(&self, direction: &Direction) -> Pos {
        match direction {
            Direction::UP => Pos {
                x: self.x,
                y: if self.y < 1 {
                    (BOARD_COLS - 1) as u16
                } else {
                    self.y - 1
                },
            },
            Direction::LEFT => Pos {
                y: self.y,
                x: if self.x < 1 {
                    (BOARD_ROWS - 1) as u16
                } else {
                    self.x - 1
                },
            },
            Direction::RIGHT => Pos {
                y: self.y,
                x: if self.x >= (BOARD_COLS - 1) as u16 {
                    0
                } else {
                    self.x + 1
                },
            },
            Direction::DOWN => Pos {
                x: self.x,
                y: if self.y >= (BOARD_ROWS - 1) as u16 {
                    0
                } else {
                    self.y + 1
                },
            },
        }
    }
}
pub struct Snake {
    pub direction: Direction,
    pub parts_x_y: Vec<Pos>,
}

impl Snake {
    pub fn set_direction(&mut self, new_direction: Direction) {
        self.direction = new_direction;
    }

    pub fn move_next(&mut self) {
        // Head
        let head = self.parts_x_y.last().unwrap();
        // New position for head
        //let new_pos = calculate_next_x_y(head.clone(), &self.direction);
        let new_pos = head.next(&self.direction);
        // Hvis hodet ender på en plass hvor det er mat, ta bort maten, og sett 10 poeng på spillet
        // New parts is eq to the old minus the last
        let mut current_parts = self.parts_x_y[1..].to_vec();
        current_parts.push(new_pos);
        self.parts_x_y = current_parts;
    }
}
