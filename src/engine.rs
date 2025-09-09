use crate::game::{Direction, Game};
use crate::graphics::draw;
use crossterm::event::{Event, KeyCode};
use crossterm::{event, execute};
use std::time::Duration;

pub fn run(mut game: Game) {
    execute!(std::io::stdout()).unwrap();
    loop {
        crossterm::terminal::enable_raw_mode().unwrap();

        // Wait at most 500ms for an event
        if event::poll(Duration::from_millis(100)).unwrap() {
            // Safe to unwrap because poll returned true
            if let Event::Key(key_event) = event::read().unwrap() {
                let next_direction = handle_keystroke(&game.snake.direction, &key_event.code);
                if next_direction.is_some() {
                    game.snake.set_direction(next_direction.unwrap());
                    game.snake.move_next();
                }

                game.update_board();
                draw(&game);
            }
        } else {
            // Either an event happened OR 500ms passed
            game.snake.move_next();
            game.update_board();
            draw(&game);
        }
    }
}

fn handle_keystroke(current_direction: &Direction, key_code: &KeyCode) -> Option<Direction> {
    match key_code {
        KeyCode::Char('w') => {
            let next = (current_direction == &Direction::LEFT
                || current_direction == &Direction::RIGHT)
                .then_some(Direction::UP);
            next
        }
        KeyCode::Char('d') => {
            let next = (current_direction == &Direction::UP
                || current_direction == &Direction::DOWN)
                .then_some(Direction::RIGHT);
            next
        }
        KeyCode::Char('a') => {
            let next = (current_direction == &Direction::UP
                || current_direction == &Direction::DOWN)
                .then_some(Direction::LEFT);
            next
        }
        KeyCode::Char('s') => {
            let next = (current_direction == &Direction::LEFT
                || current_direction == &Direction::RIGHT)
                .then_some(Direction::DOWN);
            next
        }
        KeyCode::Char('q') => {
            println!("Quitting...");
            crossterm::terminal::disable_raw_mode().unwrap();
            std::process::exit(0);
        }
        _ => None,
    }
}
