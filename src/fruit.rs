use crate::game::physics::Position;

pub struct Fruit {
    pos: Position,
}

impl Fruit {
    pub fn new() -> Self {
        Self {
            pos: Position { x: 0, y: 0 },
        }
    }
}
