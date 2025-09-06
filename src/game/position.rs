use crate::{Direction, COLS, ROWS};

#[derive(Clone, Copy)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

impl Pos {
    // Returns a new Pos with the next coordinates.
    pub fn next(&self, direction: &Direction) -> Pos {
        match direction {
            Direction::UP => Pos {
                x: self.x,
                y: if self.y < 1 {
                    (COLS - 1) as i32
                } else {
                    &self.y - 1
                },
            },
            Direction::LEFT => Pos {
                y: self.y,
                x: if self.x < 1 {
                    (ROWS - 1) as i32
                } else {
                    &self.x - 1
                },
            },
            Direction::RIGHT => Pos {
                y: self.y,
                x: if self.x >= (COLS - 1) as i32 {
                    0
                } else {
                    &self.x + 1
                },
            },
            Direction::DOWN => Pos {
                x: self.x,
                y: if self.y >= (ROWS - 1) as i32 {
                    0
                } else {
                    &self.y + 1
                },
            },
        }
    }
}
