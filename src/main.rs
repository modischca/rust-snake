#![allow(unused)] // Remove later
mod db;
mod engine;
mod game;
mod graphics;
mod test;
use game::{Cell, Game};
use rusqlite::Error;
use std::io;
fn main() {
    graphics::show_intro();
    if db::is_active_game() == false {
        let mut game = Game::new();
        game.save();
        engine::run(game);
    }
    if (recover_prev_game()) {
        let mut game = Game::new();
        game.save();
        engine::run(game);
    } else {
        let mut game = Game::new();
        game.save();
        engine::run(game);
    }
}

fn recover_prev_game() -> bool {
    println!("Do you want to continue previous game? [Y/N]");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let res = input.trim();
    match res {
        "Y" => true,
        _ => false,
    }
}
