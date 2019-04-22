use crate::{draw::*, fruit::Fruit, snake::Snake};

pub struct State {
    snake: Snake,
    fruit: Fruit,
    size: (u32, u32),
    score: u32,
    over: bool,
    paused: bool,
}

impl State {
    pub fn new() -> Self {
        Self {
            snake: Snake::new(),
            fruit: Fruit::new(),
            size: (20, 20),
            score: 0,
            over: false,
            paused: true,
        }
    }

    pub fn start(&mut self) {
        self.paused = false;
    }

    pub fn draw() {}
}
