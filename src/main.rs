use crossterm::event::{Event, KeyCode};
use crossterm::{event, execute};
use snake::{COLS, ROWS};
use std::time::Duration;
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
}

impl Game {
    pub fn start(self) -> Snake {
        let start_direction = Direction::RIGHT;
        let start_pos = Pos { x: 5, y: 5 };
        let mut parts: Vec<Pos> = vec![start_pos];
        let size = 6;
        for _i in 0..size {
            let pos = parts[_i].next(&start_direction);
            parts.push(pos);
        }

        let mut grid: [[bool; COLS]; ROWS] = [[false; COLS]; ROWS];

        for pos in parts.iter() {
            grid[pos.y as usize][pos.x as usize] = true;
        }

        let snake = Snake {
            direction: Direction::RIGHT,
            board: grid,
            parts_x_y: parts,
        };
        snake
    }
}

struct Snake {
    direction: Direction,
    board: [[bool; COLS]; ROWS],
    parts_x_y: Vec<Pos>,
}

impl Snake {
    pub fn eat(&mut self) {
        let head = self.parts_x_y.last().unwrap();
        // Double head, but will be fixed in next move.
        //self.parts_x_y.push((head.0, head.1));
    }

    pub fn set_direction(&mut self, new_direction: Direction) {
        self.direction = new_direction;
    }

    pub fn move_next(&mut self) -> [[bool; COLS]; ROWS] {
        // The new grid
        let mut grid: [[bool; COLS]; ROWS] = [[false; COLS]; ROWS];
        // Head
        let head = self.parts_x_y.last().unwrap();
        // New position for head
        //let new_pos = calculate_next_x_y(head.clone(), &self.direction);
        let new_pos = head.next(&self.direction);
        // New parts is eq to the old minus the last
        let mut current_parts = self.parts_x_y[1..].to_vec();
        current_parts.push(new_pos);
        self.parts_x_y = current_parts;

        for pos in &self.parts_x_y {
            grid[pos.y as usize][pos.x as usize] = true;
        }
        self.board = grid;
        grid
    }
}

fn main() {
    let game = Game { score: 0 };
    let snake = game.start();
    read_input_from_terminal(snake);
}

fn read_input_from_terminal(mut snake: Snake) {
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
                            graphics::draw(&snake.board);
                        }
                    }
                    KeyCode::Char('d') => {
                        if snake.direction == Direction::UP || snake.direction == Direction::DOWN {
                            snake.set_direction(Direction::RIGHT);
                            graphics::draw(&snake.board);
                        }
                    }
                    KeyCode::Char('a') => {
                        if snake.direction == Direction::UP || snake.direction == Direction::DOWN {
                            snake.set_direction(Direction::LEFT);
                            graphics::draw(&snake.board);
                        }
                    }
                    KeyCode::Char('s') => {
                        if snake.direction == Direction::LEFT || snake.direction == Direction::RIGHT
                        {
                            snake.set_direction(Direction::DOWN);
                            graphics::draw(&snake.board);
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
            }
        } else {
            // Either an event happened OR 500ms passed
            snake.move_next();
            graphics::draw(&snake.board);
        }
    }
}
