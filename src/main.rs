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

    let (player_name, got_name) = match greet() {
        Ok(name) => (name, true),
        Err(e) => {
            println!("Unable to fetch name. Starting new game!");
            ("".to_string(), false)
        }
    };

    if let (Some(current_game), input_name) = ((Game::load_existing(player_name)), got_name) {
        if prompt_recover_prev_game(&current_game) == true {
            engine::run(current_game);
        }
    }

    let mut game = Game::new(None);
    game.save();
    engine::run(game);
}

fn greet() -> Result<String, stdError> {
    println!("What is your name?");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn prompt_recover_prev_game(game: &Game) -> bool {
    println!(
        "Welcome back {}, Do you want to continue previous game? You current score is {} [Y/N]",
        game.player_name,
        game.score.to_string()
    );
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
