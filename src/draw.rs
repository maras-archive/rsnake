use crate::colors;
use crate::physics::{Direction, Position};
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

pub fn draw_snake_head(ctx: &Context, g: &mut G2d, c: Color, pos: &Position, dir: &Direction) {
    draw_block(ctx, g, c, pos);

    fn draw_eye(ctx: &Context, g: &mut G2d, x: f64, y: f64) {
        rectangle(colors::BACKGROUND, [x, y, 5.0, 5.0], ctx.transform, g);
    }

    let (x, y) = (
        blocks_in_pixels(pos.x as u32) as f64,
        blocks_in_pixels(pos.y as u32) as f64,
    );

    let block = blocks_in_pixels(1) as f64;

    match dir {
        Direction::Up => {
            draw_eye(ctx, g, x + 5.0, y + 5.0);
            draw_eye(ctx, g, x + block - 10.0, y + 5.0);
        }
        Direction::Right => {
            draw_eye(ctx, g, x + block - 10.0, y + 5.0);
            draw_eye(ctx, g, x + block - 10.0, y + block - 10.0);
        }
        Direction::Down => {
            draw_eye(ctx, g, x + 5.0, y + block - 10.0);
            draw_eye(ctx, g, x + block - 10.0, y + block - 10.0);
        }
        Direction::Left => {
            draw_eye(ctx, g, x + 5.0, y + 5.0);
            draw_eye(ctx, g, x + 5.0, y + block - 10.0);
        }
    }
}

pub fn draw_fruit(ctx: &Context, g: &mut G2d, c: Color, pos: &Position) {}

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
