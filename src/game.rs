use piston_window::*;

use crate::{draw, fruit::Fruit, snake::Snake};

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
            snake: Snake::new(),
            fruit: Fruit::new(),
            size: (width, height),
            score: 0,
            over: false,
            paused: true,
        }
    }

    // pub fn start(&mut self) {
    //     self.paused = false;
    // }

    pub fn draw() {
        println!("im drawing the game")
    }

    pub fn update() {}
}
