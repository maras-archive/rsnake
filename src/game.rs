use crate::{fruit::Fruit, snake::Snake};

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
}

pub mod physics {
    pub struct Position {
        pub x: i32,
        pub y: i32,
    }

    impl Position {
        pub fn move_by(&mut self, force: (i32, i32)) {
            self.x += force.0;
            self.y += force.1;
        }
    }

    pub enum Direction {
        Up,
        Right,
        Down,
        Left,
    }

    impl Direction {
        fn opposite(&self) -> Direction {
            match *self {
                Direction::Up => Direction::Down,
                Direction::Right => Direction::Left,
                Direction::Down => Direction::Up,
                Direction::Left => Direction::Right,
            }
        }
    }
}
