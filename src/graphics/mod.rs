use crossterm::{terminal, ExecutableCommand};
use snake::{COLS, ROWS};
use std::io::{self};
pub fn draw(datagrid: &[[bool; COLS]; ROWS]) {
    let mut stdout = io::stdout();
    stdout
        .execute(terminal::Clear(terminal::ClearType::All))
        .unwrap();
    let empty_block = '⬛';
    let non_empty_block = '🟩';
    let mut output = String::from("");
    for (index, row) in datagrid.iter().enumerate() {
        for (a, &cell) in row.iter().enumerate() {
            if cell == true {
                output.push_str(&format!("{}", non_empty_block.to_string()));
            } else {
                if index == 8 && a == 8 {
                    output.push_str(&format!("{}", "s"));
                } else {
                    output.push_str(&format!("{}", empty_block));
                }
            }
        }
        output.push_str("\r\n");
    }
    print!("{}", output);
}
