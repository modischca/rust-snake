use crate::Game;
use crossterm::style::{
    Attribute, Color, Print, ResetColor, SetAttribute, SetBackgroundColor, SetForegroundColor,
};
use crossterm::{execute, terminal, ExecutableCommand};
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
    let pos = &game.snake.parts_x_y[&game.snake.parts_x_y.len() - 1];
    output.push_str(&format!("Morten (Learn Rust) Snake"));
    output.push_str("\r\n");
    output.push_str("\r\n");
    output.push_str(&format!("Use keyboard:"));
    output.push_str("\r\n");
    output.push_str(&format!("w(up), d(right), a(left), s(down)."));
    output.push_str("\r\n");
    output.push_str("\r\n");
    output.push_str(&format!("Score: {}", game.score));
    output.push_str(&format!("         "));
    output.push_str(&format!("Snake at: {},{}", pos.x, pos.y));
    output.push_str("\r\n");
    for row in datagrid.iter() {
        for &cell in row.iter() {
            match &cell {
                Cell::EMPTY => {
                    output.push_str(&format!("{}", empty_block));
                }
                Cell::SNAKEBODY => {
                    output.push_str(&format!("{}", snake_body));
                }
                Cell::SNAKEHEAD => {
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

pub fn show_intro() {
    let snake = r#"
            /^\/^\
        _|__|  O|
\/     /~     \_/ \
    \____|__________/  \
        \_______      \
                `\     \                 \
                    |     |                  \
                    /      /                    \
                /     /                       \
                /      /                         \ \
                /     /                            \  \
            /     /             _----_            \   \
            /     /           _-~      ~-_         |   |
            (      (        _-~    _--_    ~-_     _/   |
            \      ~-____-~    _-~    ~-_    ~-_-~    /
            ~-_           _-~          ~-_       _-~
                ~--______-~                ~-___-~
    "#;
    execute!(
        io::stdout(),
        // Blue foreground
        SetForegroundColor(Color::Blue),
        // Print text
        Print(snake),
        Print(" "),
        // Reset to default colors
        Print("       Morten's learning Rust game SNAKE          "),
        ResetColor
    );
    println!("");
    println!("");
}
pub fn run_intro() {
    loop {
        sleep(Duration::from_millis(100));
        println!("Morten");
    }
}
