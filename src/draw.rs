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

pub fn draw_overlay(ctx: &Context, g: &mut G2d, c: Color, size: (u32, u32)) {
    rectangle(
        c,
        [
            0.0,
            0.0,
            blocks_in_pixels(size.0) as f64,
            blocks_in_pixels(size.1) as f64,
        ],
        ctx.transform,
        g,
    );
}

pub fn blocks_in_pixels(n: u32) -> u32 {
    n * BLOCK_SIZE as u32
}
