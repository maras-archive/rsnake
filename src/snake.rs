use piston_window::{Context, G2d};
use std::collections::LinkedList;

use crate::colors;
use crate::draw::*;
use crate::physics::{Direction, Position};

const INITIAL_SNAKE_TAIL_LENGTH: usize = 2;

pub struct Snake {
    direction: Direction,
    head: Position,
    tail: LinkedList<Position>,
    updated_tail_pos: bool,
}

impl Snake {
    pub fn new(head: Position) -> Self {
        let (x, y) = (head.x, head.y);
        let mut tail = LinkedList::new();

        for i in 1..(INITIAL_SNAKE_TAIL_LENGTH + 1) {
            tail.push_back(Position { x, y: y - i as i32 });
        }

        Self {
            direction: Direction::Down,
            head: Position { x, y },
            tail,
            updated_tail_pos: false,
        }
    }

    // pub fn dir(&self) -> Direction {
    //     self.direction
    // }

    // pub fn r#move(&mut self) {}

    // pub fn grow(&mut self, x: u32, y: u32) {
    //     self.tail.push_back(Position { x, y })
    // }
    pub fn update(&mut self, width: u32, height: u32) {
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

        if self.head.x >= width as i32 {
            self.head.x = 0;
        } else if self.head.y >= height as i32 {
            self.head.y = 0;
        } else if self.head.y < 0 {
            self.head.y = height as i32;
        } else if self.head.x < 0 {
            self.head.x = width as i32;
        }

        self.updated_tail_pos = true;
    }

    pub fn draw(&self, ctx: &Context, g: &mut G2d) {
        for block in self.tail.iter() {
            draw_block(&ctx, g, colors::SNAKE, block)
        }

        draw_snake_head(&ctx, g, colors::SNAKE, &self.head, &self.direction);
    }

    pub fn set_dir(&mut self, dir: Direction) {
        if dir == self.direction.opposite() || !self.updated_tail_pos {
            return;
        }

        self.direction = dir;
        self.updated_tail_pos = false;
    }

    pub fn get_head_pos(&self) -> &Position {
        &self.head
    }

    pub fn get_len(&self) -> usize {
        &self.tail.len() - INITIAL_SNAKE_TAIL_LENGTH
    }

    // pub fn is_alive(&self, size: (u32, u32)) -> bool {
    // let next_pos = self.next_pos();
    // let (width, height) = size;

    // next_pos.x >= 0
    //     && next_pos.y >= 0
    //     && next_pos.x <= (width - 1) as i32
    //     && next_pos.y <= (height - 1) as i32
    //     &&

    // !self.is_tail_overlapping()
    // }

    pub fn is_tail_overlapping(&self) -> bool {
        for pos in self.tail.iter() {
            if *pos == self.head {
                return true;
            }
        }

        false
    }

    pub fn will_tail_overlapp(&self) -> bool {
        let next = self.next_head_pos();

        for pos in self.tail.iter() {
            if *pos == next {
                return true;
            }
        }

        false
    }

    pub fn grow(&mut self) {
        let last = match self.tail.back() {
            Some(pos) => pos.clone(),
            None => self.head.clone(),
        };

        self.tail.push_back(last);
    }

    fn next_head_pos(&self) -> Position {
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
