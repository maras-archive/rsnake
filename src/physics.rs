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