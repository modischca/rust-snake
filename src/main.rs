#![allow(unused)] // Remove later
mod db;
mod engine;
mod game;
mod graphics;
mod test;
use game::{Cell, Game};
use rusqlite::Error;
use std::io::{self, Error as stdError};
fn main() {
    let name = graphics::show_intro();
    let player_name = greet().unwrap_or("Unable to fetch name. Starting new game.".to_string());

    if (db::get(player_name) && recover_prev_game()) {
        let mut game = Game::new();
        game.save();
        engine::run(game);
    } else {
        let mut game = Game::new();
        game.save();
        engine::run(game);
    }
}

fn greet() -> Result<String, stdError> {
    println!("What is your name?");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let res = input.trim();
    Ok(res.to_string())
}

fn recover_prev_game() -> bool {
    println!("Do you want to continue previous game? [Y/N]");
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        return false;
    }
    let res = input.trim();
    match res {
        "Y" => true,
        _ => false,
    }
}
