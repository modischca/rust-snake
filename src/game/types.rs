#[derive(Clone, Copy)]
pub enum Cell {
    EMPTY,
    SNAKE_BODY,
    SNAKE_HEAD,
    FOOD,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

pub enum GameStatus {
    RUNNING,
    END,
}

impl GameStatus {
    pub fn get_db_value(self) -> &'static str {
        match self {
            GameStatus::RUNNING => "RUNNING",
            GameStatus::END => "END",
        }
    }
}
