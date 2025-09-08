mod engine;
mod game;
mod graphics;
mod test;

use game::enums::Cell;
use game::game::Game;
use snake::{COLS, ROWS};

use std::time::SystemTime;

fn main() {
    let mut game = Game {
        score: 0,
        next_food_target: None,
        board: [[Cell::EMPTY; COLS]; ROWS],
        game_start_at: SystemTime::now(),
    };
    let snake = game.new();
    engine::run(game, snake);
}
