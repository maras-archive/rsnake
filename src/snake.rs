use crate::game::physics::Direction;

pub struct Snake {
    direction: Direction,
}

impl Snake {
    pub fn new() -> Self {
        Self {
            direction: Direction::Left,
        }
    }
}
