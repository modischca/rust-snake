mod engine;
mod game;
mod graphics;
mod test;

use game::{Cell, Game};

fn main() {
    engine::run(Game::new());
}
