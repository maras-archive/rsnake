use piston_window::*;

use crate::{colors, draw::*, fruit::Fruit, physics::Position, snake::Snake};

pub struct Game {
    snake: Snake,
    fruit: Fruit,
    size: (u32, u32),
    score: u32,
    over: bool,
    paused: bool,
}

impl Game {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            snake: Snake::new(5, 5),
            fruit: Fruit::new(),
            size: (width, height),
            score: 0,
            over: false,
            paused: true,
        }
    }

    pub fn start(&mut self) {
        self.paused = false;
    }

    pub fn draw(&self, ctx: Context, g: &mut G2d) {
        self.draw_border(&ctx, g);
        self.snake.draw(&ctx, g);
    }

    pub fn update(&mut self) {}

    fn draw_border(&self, ctx: &Context, g: &mut G2d) {
        let [win_width, win_height] = ctx.get_view_size();

        rectangle(
            colors::BORDER,
            [0.0, 0.0, BLOCK_SIZE, win_height],
            ctx.transform,
            g,
        );

        rectangle(
            colors::BORDER,
            [0.0, 0.0, win_width, BLOCK_SIZE],
            ctx.transform,
            g,
        );

        rectangle(
            colors::BORDER,
            [win_width - BLOCK_SIZE, 0.0, BLOCK_SIZE, win_height],
            ctx.transform,
            g,
        );

        rectangle(
            colors::BORDER,
            [0.0, win_height - BLOCK_SIZE, win_width, BLOCK_SIZE],
            ctx.transform,
            g,
        );
    }
}
