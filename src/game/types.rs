#[derive(Clone, Copy)]
pub enum Cell {
    EMPTY,
    SNAKEBODY,
    SNAKEHEAD,
    FOOD(&'static str),
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


pub enum FoodType {
    APPLE,
    BANANA,
    CHICKEN,
    DONUT
}

impl FoodType {
    pub fn get_emoji(&self) -> &'static str {
        match &self {
            FoodType::APPLE => {
                "🍎"
            },
            FoodType::BANANA => {
                "🍌"
            },
            FoodType::CHICKEN => {
                "🍗"
            },
            FoodType::DONUT => {
                "🍩"
            }
            _ => {
                "🍔"
            }
        }
    }

    pub fn random () -> FoodType {
        let random_number = rand::random_range(0..2);
        match random_number {
            0 => FoodType::APPLE,
            _ => FoodType::BANANA
        }
    }
}