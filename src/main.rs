#![allow(unused)] // Remove later
use std::io::Write;
mod db;
mod engine;
mod errors;
mod game;
mod graphics;
mod logger;
mod test;
use crate::errors::GameErr;
use game::{Cell, Game};
use logger::{log, Log};
use rusqlite::Error;
use std::io::{self, Error as stdError};
fn main() {
    if let Err(e) = logger::setup_logger() {
        println!("Unable to initlaize logger. Reason: {}", e);
    }
    let name = graphics::show_intro();
    setup_game();
}

fn setup_game() {
    let (player_name, got_name) = match greet() {
        Ok(name) => (name, true),
        Err(e) => {
            println!("Unable to fetch name. Starting new game!");
            ("".to_string(), false)
        }
    };

    if let (Some(current_game), _) = ((Game::load_existing(player_name)), got_name) {
        if prompt_recover_prev_game(&current_game) == true {
            println!("Starting game at score {}", &current_game.score.to_string());
            play(current_game);
        }
    }

    println!("Starting new game");
    let mut new_game = Game::new(None);
    new_game.save();
    play(new_game);
}

fn play(game: Game) {
    log(Log::INFO("New game started".into()));
    if let Err(e) = engine::run(game) {
        handle_game_error(e);
    } else {
        quit();
    }
}

fn handle_game_error(e: GameErr) {
    if crossterm::terminal::disable_raw_mode().is_err() {
        eprintln!("Unable to disable raw mode.");
    }
    match e {
        GameErr::SnakeCrashedIntoItself => {
            println!("Game over.");
        }
        _ => {
            println!("Unhandled error. Terminating.");
        }
    }
    io::stdout().flush().ok();
    quit();
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

fn quit() {
    println!("Game shut down.");
    std::process::exit(0);
}
