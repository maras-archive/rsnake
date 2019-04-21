pub struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn move_by(&mut self, force: (i32, i32)) {
        self.x += force.0;
        self.y += force.1;
    }
}
