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
        draw_block(&ctx, g, colors::SNAKE, &self.head);

        for block in self.tail.iter() {
            draw_block(&ctx, g, colors::SNAKE, block)
        }
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

    pub fn is_tail_overlapping(&self) -> bool {
        for pos in self.tail.iter() {
            if *pos == self.head {
                return true;
            }
        }

        false
    }
}
