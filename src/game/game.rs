use crate::game::{Cell, Direction, Pos, Snake};
use crate::{COLS, ROWS};

pub struct Game {
    pub score: i32,
    pub next_food_target: Option<Pos>,
    pub board: [[Cell; COLS]; ROWS],
    pub game_start_at: std::time::SystemTime,
}

impl Game {
    pub fn new(&mut self) -> Snake {
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
        self.update_board(&snake);
        snake
    }

    pub fn update_board(&mut self, snake: &Snake) {
        // The new grid
        if (self.next_food_target.is_some()) {
            let a = snake.parts_x_y.iter().find(|p| {
                p.x == self.next_food_target.unwrap().x && p.y == self.next_food_target.unwrap().y
            });
            if a.is_some() {
                self.score = self.score + 10;
                self.next_food_target = None;
            }
        }

        let mut grid: [[Cell; COLS]; ROWS] = [[Cell::EMPTY; COLS]; ROWS];

        for pos in &snake.parts_x_y {
            grid[pos.y as usize][pos.x as usize] = Cell::SNAKE_BODY;
        }
        let head = &snake.parts_x_y.iter().last().unwrap();
        grid[head.y as usize][head.x as usize] = Cell::SNAKE_HEAD;

        if self.next_food_target.is_none() {
            // Add food
            let x = rand::random_range(0..ROWS - 1);
            let y = rand::random_range(0..COLS - 1);
            let food_at_pos = Pos {
                x: x as i32,
                y: y as i32,
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
