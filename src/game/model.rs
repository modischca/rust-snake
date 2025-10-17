use crate::db;
use crate::errors::{GameErr, GameResult};
use crate::game::types::{FoodType, GameStatus};
use crate::game::{Cell, Direction};
pub const BOARD_COLS: usize = 16;
pub const BOARD_ROWS: usize = 16;
use std::time::SystemTime;

pub trait CurrentPos {
    fn get_pos(&self) -> &Pos;
}


pub struct Food {
    pos: Pos,
    icon: FoodType
}

impl CurrentPos for Food {
    fn get_pos(&self) -> &Pos {
        &self.pos
    }
}

pub struct Board {
    pub board: [[Cell; BOARD_COLS]; BOARD_ROWS],
}

impl Board {
    pub fn new() -> Self {
        let mut board: [[Cell; BOARD_COLS]; BOARD_ROWS] = [[Cell::EMPTY; BOARD_COLS]; BOARD_ROWS];
        for i in 0..BOARD_ROWS {
            for j in 0..BOARD_COLS {
                board[i][j] = Cell::EMPTY;
            }
        }
        Self { board }
    }

    pub fn clear(&mut self) {
        for i in 0..BOARD_ROWS {
            for j in 0..BOARD_COLS {
                self.board[i][j] = Cell::EMPTY;
            }
        }
    }

    pub fn is_pos_available(&self, pos: &Pos) -> bool {
        let cell_type = self.board[pos.y as usize][pos.x as usize];
        match cell_type {
            Cell::EMPTY => true,
            _ => false,
        }
    }
}
pub struct Game {
    pub score: u16,
    //pub next_food_target: Option<Pos>,
    pub food: Vec<Food>,
    pub board: Board,
    pub game_start_at: std::time::SystemTime,
    pub snake: Snake,
    pub guests: Vec<Snake>,
    pub db_id: Option<u16>,
    pub game_status: GameStatus,
    pub player_name: String,
}

impl Game {
    pub fn new(player_name: Option<String>) -> Self {
        let pn = player_name.unwrap_or("Unknown".to_string());
        Self {
            score: 0,
            food: Vec::new(),
            board: Board::new(),
            game_start_at: SystemTime::now(),
            snake: Snake::new(None, Direction::RIGHT),
            db_id: None,
            game_status: GameStatus::RUNNING,
            player_name: pn,
            guests: vec![],
        }
    }

    pub fn add_guest(&mut self) {
        self.guests.push(Snake::new(None, Direction::DOWN));
    }

    pub fn save(&mut self) -> Result<(), rusqlite::Error> {
        if self.db_id.is_none() {
            db::insert(self)
        } else {
            db::update(&self)
        }
    }

    pub fn load_existing(player_name: String) -> Option<Game> {
        match db::get(player_name) {
            Ok(game) => Some(game),
            Err(_e) => None,
        }
    }

    pub fn update_score(&mut self) {
        // Just tryin the let else pattern.
        let snake_pos = self.snake.get_pos().clone();
        let index = self
            .food
            .iter()
            .position(|f| f.pos.x == snake_pos.x && f.pos.y == snake_pos.y);
        if let Some(index) = index {
            self.score += 10;
            self.food.remove(index);
            self.snake.grow();
        }
    }

    pub fn update_board(&mut self) {
        self.update_score();
        self.board.clear();

        for snake in std::iter::once(&mut self.snake).chain(self.guests.iter_mut()) {
            for (i, pos) in snake.parts_x_y.iter().enumerate() {
                let cell = if i == snake.length() - 1 { Cell::SNAKEHEAD } else { Cell::SNAKEBODY };
                self.board.board[pos.y as usize][pos.x as usize] = cell;
            }
        }
        self.handle_food();
    }
    fn handle_food(&mut self) {

        if self.food.len() > 0 {
            // Place food at board if food added to the game.
            for f in &self.food {
                self.board.board[f.pos.y as usize][f.pos.x as usize] = Cell::FOOD(f.icon.get_emoji());
            }
        } else {
            let number_of_food
                = rand::random_range(0..5);
            for _i in 0..number_of_food {
                // Add the next food position to the game if there is no food.
                // Food will be drawn on board on the next iteration.
                let food_pod = Pos {
                    x: rand::random_range(0..BOARD_ROWS - 1) as u16,
                    y: rand::random_range(0..BOARD_COLS - 1) as u16,
                };
                if (self.board.is_pos_available(&food_pod)) {
                    self.food.push(Food {
                        icon: FoodType::random(),
                        pos: food_pod,
                    });
                }
            }
        }
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

impl CurrentPos for Snake {
    fn get_pos(&self) -> &Pos {
        self.parts_x_y.last().expect("Snake need to have a head")
    }
}

impl Snake {
    pub fn set_direction(&mut self, new_direction: Direction) {
        self.direction = new_direction;
    }

    pub fn move_next(&mut self) -> GameResult<()> {
        // Ta bort bakerste posisjon, og legg til nytt pos på hodet.
        let head = self.parts_x_y.last().expect("Snake always has a head");
        let new_pos = head.next(&self.direction);

        let mut current_parts = self.parts_x_y[1..].to_vec();

        if current_parts
            .iter()
            .any(|f| f.x == new_pos.x && f.y == new_pos.y)
        {
            return Err(GameErr::SnakeCrashedIntoItself);
        }

        current_parts.push(new_pos);
        self.parts_x_y = current_parts;
        Ok(())
    }

    pub fn grow(&mut self) {
        let current_size = self.length();
        let head = self.parts_x_y.last().expect("Snake always has a head");
        let new_pos = head.next(&self.direction);
        self.parts_x_y.push(new_pos);
    }

    pub fn length(&self) -> usize {
        self.parts_x_y.len()
    }

    pub fn new(score: Option<usize>, direction: Direction) -> Self {
        let start_direction = &direction;
        let start_pos = Pos { x: 5, y: 5 };
        let mut parts: Vec<Pos> = vec![start_pos];
        let mut size = score.unwrap_or(0);
        if (size > 0) {
            size = size / 10;
        }
        size = size + 6;
        for _i in 0..size {
            let pos = parts[_i].next(&start_direction);
            parts.push(pos);
        }
        Self {
            direction: direction,
            parts_x_y: parts,
        }
    }
}
