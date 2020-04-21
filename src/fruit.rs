use piston_window::{Context, G2d};

use crate::draw::draw_fruit;
use crate::physics::Position;

pub struct Fruit(Position);

impl Fruit {
    pub fn new(position: Position) -> Self {
        Self(position)
    }

    pub fn get_pos(&self) -> Position {
        self.0
    }

    pub fn draw(&self, ctx: &Context, g: &mut G2d) {
        draw_fruit(ctx, g, &self.0)
    }
}
