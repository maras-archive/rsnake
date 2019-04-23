use crate::physics::Position;
use piston_window::types::Color;
use piston_window::{rectangle, Context, G2d};

pub const BLOCK_SIZE: f64 = 25.0;

pub fn draw_block(ctx: &Context, g: &mut G2d, c: Color, pos: &Position) {
    rectangle(
        c,
        [
            pos.x as f64 * BLOCK_SIZE,
            pos.y as f64 * BLOCK_SIZE,
            BLOCK_SIZE,
            BLOCK_SIZE,
        ],
        ctx.transform,
        g,
    );
}
