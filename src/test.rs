#[cfg(test)]
mod test {
    use super::*;
    use crate::*;

    #[test]
    fn move_to_right() {
        let p = Pos { x: 1, y: 1 };
        let go_right = Direction::RIGHT;
        let pos = p.next(&go_right);
        let expected_resut = Pos { x: 2, y: 1 };
        assert_eq!(pos.x, expected_resut.x);
        assert_eq!(pos.y, expected_resut.y);
    }

    #[test]
    fn move_right_and_into_left() {
        let p = Pos {
            x: COLS as i32,
            y: 1,
        };
        let go_right = Direction::RIGHT;
        let pos = p.next(&go_right);
        let expected_resut = Pos { x: 0, y: 1 };
        assert_eq!(pos.x, expected_resut.x);
        assert_eq!(pos.y, expected_resut.y);
    }

    #[test]
    fn move_left_and_into_right() {
        let p = Pos { x: 0, y: 1 };
        let go_right = Direction::LEFT;
        let pos = p.next(&go_right);
        let expected_resut = Pos {
            x: (COLS - 1) as i32,
            y: 1,
        };
        assert_eq!(pos.x, expected_resut.x);
        assert_eq!(pos.y, expected_resut.y);
    }

    #[test]
    fn move_down() {
        let p = Pos { x: 0, y: 1 };
        let go_right = Direction::DOWN;
        let pos = p.next(&go_right);
        let expected_resut = Pos { x: 0, y: 2 };
        assert_eq!(pos.x, expected_resut.x);
        assert_eq!(pos.y, expected_resut.y);
    }

    #[test]
    fn move_down_into_top() {
        let p = Pos {
            x: 1,
            y: (ROWS - 1) as i32,
        };
        let go_down = Direction::DOWN;
        let pos = p.next(&go_down);
        let expected_resut = Pos { x: 1, y: 0 };
        assert_eq!(pos.x, expected_resut.x);
        assert_eq!(pos.y, expected_resut.y);
    }

    #[test]
    fn move_top_into_bottom() {
        let p = Pos { x: 1, y: 0 };
        let go_up = Direction::UP;
        let pos = p.next(&go_up);
        let expected_resut = Pos {
            x: 1,
            y: (ROWS - 1) as i32,
        };
        assert_eq!(pos.x, expected_resut.x);
        assert_eq!(pos.y, expected_resut.y);
    }
}
