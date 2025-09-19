use crate::db;
use crate::game::{Direction, Game};
use crate::graphics::draw;
use crossterm::event::{Event, KeyCode};
use crossterm::{event, execute};
use std::time::Duration;

enum KeyStrokeResult {
    SetNewDirection(Direction),
    Quit,
}
pub fn run(mut game: Game) {
    execute!(std::io::stdout()).unwrap();
    loop {
        crossterm::terminal::enable_raw_mode().unwrap();

        // Wait at most 500ms for an event
        if event::poll(Duration::from_millis(100)).unwrap() {
            // Safe to unwrap because poll returned true
            if let Event::Key(key_event) = event::read().unwrap() {
                let next_direction = handle_keystroke(&game.snake.direction, &key_event.code);
                match next_direction {
                    Some(KeyStrokeResult::Quit) => {
                        game.save();
                        quit();
                    }
                    Some(KeyStrokeResult::SetNewDirection(direction)) => {
                        game.snake.set_direction(direction);
                        game.snake.move_next();
                        game.set_score();
                    }
                    _ => {}
                }

                game.update_board();
                draw(&game);
            }
        } else {
            game.snake.move_next();
            game.set_score();
            game.update_board();
            draw(&game);
        }
    }
}

fn quit() {
    println!("Quitting...");
    crossterm::terminal::disable_raw_mode().unwrap();
    std::process::exit(0);
}

fn handle_keystroke(current_direction: &Direction, key_code: &KeyCode) -> Option<KeyStrokeResult> {
    match key_code {
        KeyCode::Char('w') => {
            let next = (current_direction == &Direction::LEFT
                || current_direction == &Direction::RIGHT)
                .then_some(KeyStrokeResult::SetNewDirection(Direction::UP));
            next
        }
        KeyCode::Char('d') => {
            let next = (current_direction == &Direction::UP
                || current_direction == &Direction::DOWN)
                .then_some(KeyStrokeResult::SetNewDirection(Direction::RIGHT));
            next
        }
        KeyCode::Char('a') => {
            let next = (current_direction == &Direction::UP
                || current_direction == &Direction::DOWN)
                .then_some(KeyStrokeResult::SetNewDirection(Direction::LEFT));
            next
        }
        KeyCode::Char('s') => {
            let next = (current_direction == &Direction::LEFT
                || current_direction == &Direction::RIGHT)
                .then_some(KeyStrokeResult::SetNewDirection(Direction::DOWN));
            next
        }
        KeyCode::Char('q') => Some(KeyStrokeResult::Quit),
        _ => None,
    }
}
