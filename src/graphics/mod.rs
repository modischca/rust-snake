use crossterm::{terminal, ExecutableCommand};
use snake::{COLS, ROWS};
use std::io::{self};
pub fn draw(datagrid: &[[bool; COLS]; ROWS]) {
    let mut stdout = io::stdout();
    stdout
        .execute(terminal::Clear(terminal::ClearType::All))
        .unwrap();
    let empty_block = '\u{25A1}';
    let non_empty_block = '\u{25A0}';
    let mut output = String::from("");
    for row in datagrid.iter() {
        for &cell in row {
            if cell == true {
                output.push_str(&format!("{}", non_empty_block.to_string()));
            } else {
                output.push_str(&format!("{}", empty_block));
            }
        }
        output.push_str("\r\n");
    }
    print!("{}", output);
}
