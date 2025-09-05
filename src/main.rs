use crossterm::event::{Event, KeyCode};
use crossterm::{event, execute};
use snake::{COLS, ROWS};
use std::time::Duration;
use std::time::SystemTime;
#[derive(Clone, Copy)]
enum Cell {
    EMPTY,
    SNAKE_BODY,
    SNAKE_HEAD,
    FOOD,
}

#[derive(Clone, PartialEq, Debug)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}
mod graphics;
mod test;

#[derive(Clone, Copy)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    // Returns a new Pos with the next coordinates.
    pub fn next(&self, direction: &Direction) -> Pos {
        match direction {
            Direction::UP => Pos {
                x: self.x,
                y: if self.y < 1 {
                    (COLS - 1) as i32
                } else {
                    &self.y - 1
                },
            },
            Direction::LEFT => Pos {
                y: self.y,
                x: if self.x < 1 {
                    (ROWS - 1) as i32
                } else {
                    &self.x - 1
                },
            },
            Direction::RIGHT => Pos {
                y: self.y,
                x: if self.x >= (COLS - 1) as i32 {
                    0
                } else {
                    &self.x + 1
                },
            },
            Direction::DOWN => Pos {
                x: self.x,
                y: if self.y >= (ROWS - 1) as i32 {
                    0
                } else {
                    &self.y + 1
                },
            },
        }
    }
}
struct Game {
    score: i32,
    next_food_target: Option<Pos>,
    board: [[Cell; COLS]; ROWS],
    game_start_at: std::time::SystemTime,
}

impl Game {
    pub fn start(&mut self) -> Snake {
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

struct Snake {
    direction: Direction,
    parts_x_y: Vec<Pos>,
}

impl Snake {
    pub fn eat(&mut self) {
        // if head is at same position as food
        // add score and length to snake
    }

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

fn main() {
    let mut game = Game {
        score: 0,
        next_food_target: None,
        board: [[Cell::EMPTY; COLS]; ROWS],
        game_start_at: SystemTime::now(),
    };
    let snake = game.start();
    read_input_from_terminal(game, snake);
}

fn read_input_from_terminal(mut game: Game, mut snake: Snake) {
    execute!(std::io::stdout()).unwrap();
    loop {
        crossterm::terminal::enable_raw_mode().unwrap();

        // Wait at most 500ms for an event
        if event::poll(Duration::from_millis(100)).unwrap() {
            // Safe to unwrap because poll returned true
            if let Event::Key(key_event) = event::read().unwrap() {
                match key_event.code {
                    KeyCode::Char('w') => {
                        if snake.direction == Direction::LEFT || snake.direction == Direction::RIGHT
                        {
                            snake.set_direction(Direction::UP);
                        }
                    }
                    KeyCode::Char('d') => {
                        if snake.direction == Direction::UP || snake.direction == Direction::DOWN {
                            snake.set_direction(Direction::RIGHT);
                        }
                    }
                    KeyCode::Char('a') => {
                        if snake.direction == Direction::UP || snake.direction == Direction::DOWN {
                            snake.set_direction(Direction::LEFT);
                        }
                    }
                    KeyCode::Char('s') => {
                        if snake.direction == Direction::LEFT || snake.direction == Direction::RIGHT
                        {
                            snake.set_direction(Direction::DOWN);
                        }
                    }
                    KeyCode::Char('c') => {
                        snake.eat();
                    }
                    KeyCode::Char('q') => {
                        println!("Quitting...");
                        crossterm::terminal::disable_raw_mode().unwrap();
                        std::process::exit(0);
                    }
                    _ => {}
                }
                game.update_board(&snake);
                graphics::draw(&game);
            }
        } else {
            // Either an event happened OR 500ms passed
            snake.move_next();
            game.update_board(&snake);
            graphics::draw(&game);
        }
    }
}
