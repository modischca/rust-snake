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
    pub fn new(player_name: Option<String>) -> Self {
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
        let pn = player_name.unwrap_or("Unknown".to_string());
        Self {
            score: 0,
            next_food_target: None,
            board: [[Cell::EMPTY; BOARD_COLS]; BOARD_ROWS],
            game_start_at: SystemTime::now(),
            snake: snake,
            db_id: None,
            game_status: GameStatus::RUNNING,
            player_name: pn,
        }
    }

    pub fn save(&mut self) {
        if self.db_id.is_none() {
            db::insert(self);
        } else {
            match db::update(&self) {
                Err(e) => {
                    println!("Unable to save due to: {}", e);
                }
                Ok(()) => {
                    println!("Game saved.");
                }
            }
        }
    }

    pub fn load_existing(player_name: String) -> Self {
        match db::get(player_name) {
            Ok(game) => game,
            Err(e) => {
                println!("Unable to find game. Starting a new one...");
                Game::new(None)
            }
        }
    }

    pub fn set_score(&mut self) {
        // Er det noe mat på brettet?
        // Reafactor using match
        //
        /*if let Some(food_pos) = self.next_food_target {
            if let Some(head_pos) = self.snake.parts_x_y.last() {
                if (head_pos.x == food_pos.x && head_pos.y == food_pos.y) {
                    self.score = self.score + 10;
                    self.next_food_target = None;
                }
            } else {
                eprintln!("Unable to update score.")
            }
        }*/
        if let (Some(food_pos), Some(head_pos)) =
            (self.next_food_target, self.snake.parts_x_y.last())
        {
            if head_pos.x == food_pos.x && head_pos.y == food_pos.y {
                self.score += 10;
                self.next_food_target = None;
            }
        }
    }

    pub fn update_board(&mut self) {
        let mut grid: [[Cell; BOARD_COLS]; BOARD_ROWS] = [[Cell::EMPTY; BOARD_COLS]; BOARD_ROWS];

        for pos in &self.snake.parts_x_y {
            grid[pos.y as usize][pos.x as usize] = Cell::SNAKE_BODY;
        }
        let head = self
            .snake
            .parts_x_y
            .last()
            .expect("Snake must have a head pisition.");
        grid[head.y as usize][head.x as usize] = Cell::SNAKE_HEAD;

        if let Some(next_food_target) = self.next_food_target {
            grid[next_food_target.y as usize][next_food_target.x as usize] = Cell::FOOD;
        } else {
            // Add food
            let x = rand::random_range(0..BOARD_ROWS - 1);
            let y = rand::random_range(0..BOARD_COLS - 1);
            let food_at_pos = Pos {
                x: x as u16,
                y: y as u16,
            };
            self.next_food_target = Some(food_at_pos);
            grid[y][x] = Cell::FOOD;
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
        // Ta bort bakerste posisjon, og legg til nytt pos på hodet.
        let head = self.parts_x_y.last().expect("Snake always has a head");

        let new_pos = head.next(&self.direction);

        let mut current_parts = self.parts_x_y[1..].to_vec();
        current_parts.push(new_pos);
        self.parts_x_y = current_parts;
    }

    pub fn grow(&mut self) {}
}
