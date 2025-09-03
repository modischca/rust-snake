#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_snake_move() {
        let mut snake = Snake {
            size: 1,
            x: 0,
            y: 0,
            board: [[false; COLS]; ROWS],
        };

        let mut grid = snake.move_snake(&Direction::RIGHT); // MOVE ONE DOWN
        let expected_grid = [
            [false, true, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false],
        ];
        assert_eq!(grid, expected_grid);

        grid = snake.move_snake(&Direction::RIGHT); // MOVE ONE DOWN
        let expected_grid = [
            [false, false, true, false, false, false, false, false],
            [false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false],
        ];
        assert_eq!(grid, expected_grid);

        grid = snake.move_snake(&Direction::LEFT); // MOVE ONE DOWN
        let expected_grid = [
            [false, true, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false],
        ];
        assert_eq!(grid, expected_grid);

        grid = snake.move_snake(&Direction::UP); // MOVE ONE DOWN
        let expected_grid = [
            [false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false],
            [false, true, false, false, false, false, false, false],
        ];
        assert_eq!(grid, expected_grid);

        grid = snake.move_snake(&Direction::UP); // MOVE ONE DOWN
        let expected_grid = [
            [false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false],
            [false, true, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false],
        ];
        assert_eq!(grid, expected_grid);

        snake.move_snake(&Direction::DOWN);
        grid = snake.move_snake(&Direction::DOWN);
        let expected_grid = [
            [false, true, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false],
        ];
        assert_eq!(grid, expected_grid);
    }

    #[test]
    fn grow() {
        let mut snake = Snake {
            size: 1,
            x: 0,
            y: 0,
            board: [[false; COLS]; ROWS],
        };
        snake.grow(1);
        assert_eq!(snake.size, 2);
    }
}
