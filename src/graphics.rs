use crate::Game;
use crossterm::{terminal, ExecutableCommand};
use std::{
    io::{self},
    thread::sleep,
    time::Duration,
};

use crate::Cell;
pub fn draw(game: &Game) {
    let datagrid = &game.board;
    let mut stdout = io::stdout();
    stdout
        .execute(terminal::Clear(terminal::ClearType::All))
        .unwrap();
    let empty_block = '⬛';
    let snake_body = '🟢';
    let snake_head = '🟢';
    let food = '🍔';
    let mut output = String::from("");
    output.push_str(&format!("Morten (Learn Rust) Snake"));
    output.push_str("\r\n");
    output.push_str("\r\n");
    output.push_str(&format!("Use keyboard:"));
    output.push_str("\r\n");
    output.push_str(&format!("w(up), d(right), a(left), s(down)."));
    output.push_str("\r\n");
    output.push_str("\r\n");
    output.push_str(&format!("Score: {}", game.score));
    output.push_str("\r\n");
    for row in datagrid.iter() {
        for &cell in row.iter() {
            match &cell {
                Cell::EMPTY => {
                    output.push_str(&format!("{}", empty_block));
                }
                Cell::SNAKE_BODY => {
                    output.push_str(&format!("{}", snake_body));
                }
                Cell::SNAKE_HEAD => {
                    output.push_str(&format!("{}", snake_head));
                }
                Cell::FOOD => {
                    output.push_str(&format!("{}", food));
                }
            }
        }
        output.push_str("\r\n");
    }
    print!("{}", output);
}

pub fn run_intro() {
    loop {
        sleep(Duration::from_millis(100));
        println!("Morten");
    }
}
