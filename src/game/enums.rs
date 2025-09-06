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
