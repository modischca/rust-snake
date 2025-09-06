mod game;
mod graphics;
mod test;

use crossterm::event::{Event, KeyCode};
use crossterm::{event, execute};
use game::{Cell, Direction, Game, Pos, Snake};
use snake::{COLS, ROWS};
use std::time::Duration;
use std::time::SystemTime;

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
