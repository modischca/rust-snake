use crate::{Direction, Pos};
pub struct Snake {
    pub direction: Direction,
    pub parts_x_y: Vec<Pos>,
}

impl Snake {
    pub fn eat(&mut self) {
        // if head is at same position as food
        // add score and length to snake
    }

    pub fn set_direction(&mut self, new_direction: Direction) {
        self.direction = new_direction;
    }

    pub fn move_next(&mut self) {
        // Head
        let head = self.parts_x_y.last().unwrap();
        // New position for head
        //let new_pos = calculate_next_x_y(head.clone(), &self.direction);
        let new_pos = head.next(&self.direction);
        // Hvis hodet ender på en plass hvor det er mat, ta bort maten, og sett 10 poeng på spillet
        // New parts is eq to the old minus the last
        let mut current_parts = self.parts_x_y[1..].to_vec();
        current_parts.push(new_pos);
        self.parts_x_y = current_parts;
    }
}
