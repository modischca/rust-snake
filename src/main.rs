use crossterm::event::{Event, KeyCode};
use crossterm::{event, execute, terminal, ExecutableCommand};
use std::io::{self};
use std::time::Duration;

#[derive(Clone, PartialEq, Debug)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}
mod test;

const COLS: usize = 12;
const ROWS: usize = 12;

struct Game {
    score: i32,
}

impl Game {
    pub fn start(self) -> Snake {
        let start_direction = Direction::RIGHT;
        let mut parts: Vec<(i32, i32)> = vec![];
        let size = 4;
        for _i in 0..size {
            let pos = calculate_next_x_y(5, 1, &start_direction);
            parts.push((pos.0, pos.1));
        }

        let mut grid: [[bool; COLS]; ROWS] = [[false; COLS]; ROWS];

        for xy in parts.iter() {
            grid[xy.1 as usize][xy.0 as usize] = true;
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
    parts_x_y: Vec<(i32, i32)>,
}

impl Snake {
    pub fn eat(&mut self) {
        let head = self.parts_x_y.last().unwrap();
        // Double head, but will be fixed in next move.
        self.parts_x_y.push((head.0, head.1));
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
        let new_pos = calculate_next_x_y(head.0, head.1, &self.direction);
        // New parts is eq to the old minus the last
        let mut current_parts = self.parts_x_y[1..].to_vec();
        current_parts.push((new_pos.0, new_pos.1));
        self.parts_x_y = current_parts;

        for pos in &self.parts_x_y {
            grid[pos.1 as usize][pos.0 as usize] = true;
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
                            draw(&snake.board);
                        }
                    }
                    KeyCode::Char('d') => {
                        if snake.direction == Direction::UP || snake.direction == Direction::DOWN {
                            snake.set_direction(Direction::RIGHT);
                            draw(&snake.board);
                        }
                    }
                    KeyCode::Char('a') => {
                        if snake.direction == Direction::UP || snake.direction == Direction::DOWN {
                            snake.set_direction(Direction::LEFT);
                            draw(&snake.board);
                        }
                    }
                    KeyCode::Char('s') => {
                        if snake.direction == Direction::LEFT || snake.direction == Direction::RIGHT
                        {
                            snake.set_direction(Direction::DOWN);
                            draw(&snake.board);
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
            draw(&snake.board);
        }
    }
}

fn draw(datagrid: &[[bool; COLS]; ROWS]) {
    let mut stdout = io::stdout();
    stdout
        .execute(terminal::Clear(terminal::ClearType::All))
        .unwrap();
    let empty_block = '\u{25A1}';
    let non_empty_block = '\u{25A0}';
    let mut output = String::from("");
    for row in datagrid.iter() {
        for &cell in row {
            if cell == true {
                output.push_str(&format!("{}", non_empty_block.to_string()));
            } else {
                output.push_str(&format!("{}", empty_block));
            }
        }
        output.push_str("\r\n");
    }
    print!("{}", output);
}

fn calculate_next_x_y(mut x: i32, mut y: i32, direction: &Direction) -> (i32, i32) {
    match direction {
        Direction::UP => {
            if y < 1 {
                y = (COLS - 1) as i32;
            } else {
                y = &y - 1;
            }
        }
        Direction::LEFT => {
            if x < 1 {
                x = (ROWS - 1) as i32;
            } else {
                x = &x - 1;
            }
        }
        Direction::RIGHT => {
            if x >= (COLS - 1) as i32 {
                x = 0;
            } else {
                x = &x + 1;
            }
        }
        Direction::DOWN => {
            if y >= (ROWS - 1) as i32 {
                y = 0;
            } else {
                y = &y + 1;
            }
        }
    };
    (x, y)
}
