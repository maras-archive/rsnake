use piston_window::{Context, G2d};
use std::collections::LinkedList;

use crate::colors;
use crate::draw::*;
use crate::physics::{Direction, Position};

pub struct Snake {
    direction: Direction,
    head: Position,
    tail: LinkedList<Position>,
}

impl Snake {
    pub fn new(x: i32, y: i32) -> Self {
        let mut tail = LinkedList::new();

        tail.push_back(Position { x, y: y - 1 });
        tail.push_back(Position { x, y: y - 2 });
        tail.push_back(Position { x, y: y - 3 });
        tail.push_back(Position { x, y: y - 4 });
        tail.push_back(Position { x, y: y - 5 });
        tail.push_back(Position { x, y: y - 8 });

        Self {
            direction: Direction::Down,
            head: Position { x, y },
            tail,
        }
    }

    // pub fn dir(&self) -> Direction {
    //     self.direction
    // }

    // pub fn r#move(&mut self) {}

    // pub fn grow(&mut self, x: u32, y: u32) {
    //     self.tail.push_back(Position { x, y })
    // }
    pub fn update(&mut self) {
        if self.tail.len() > 0 {
            self.tail.push_front(self.head.clone());
            self.tail.pop_back();
        }

        match self.direction {
            Direction::Up => self.head.y -= 1,
            Direction::Right => self.head.x += 1,
            Direction::Down => self.head.y += 1,
            Direction::Left => self.head.x -= 1,
        }
    }

    pub fn draw(&self, ctx: &Context, g: &mut G2d) {
        for block in self.tail.iter() {
            draw_block(&ctx, g, colors::SNAKE, block)
        }

        draw_block(&ctx, g, colors::FRUIT, &self.head);
    }

    pub fn set_dir(&mut self, dir: Direction) {
        if dir == self.direction.opposite() {
            return;
        }

        self.direction = dir;
    }

    pub fn get_head_pos(&self) -> &Position {
        &self.head
    }

    pub fn is_alive(&self, size: (u32, u32)) -> bool {
        let next_pos = self.next_pos();
        let (width, height) = size;

        next_pos.x >= 0
            && next_pos.y >= 0
            && next_pos.x <= (width - 1) as i32
            && next_pos.y <= (height - 1) as i32
            && !self.is_tail_overlapping()
    }

    fn is_tail_overlapping(&self) -> bool {
        for pos in self.tail.iter() {
            if *pos == self.head {
                return true;
            }
        }

        false
    }

    fn next_pos(&self) -> Position {
        let mut pos = self.head.clone();

        match self.direction {
            Direction::Up => pos.y -= 1,
            Direction::Left => pos.x -= 1,
            Direction::Down => pos.y += 1,
            Direction::Right => pos.x += 1,
        }

        pos
    }
}
