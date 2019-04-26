use piston_window::*;
use rand::Rng;

use crate::{
    colors, draw,
    physics::{Direction, Position},
    snake::Snake,
};

const FPS: f64 = 8.0;
// const RESTART_TIME: f64 = 1.0;

fn fps_in_ms(fps: f64) -> f64 {
    1.0 / fps
}

fn calc_random_pos(width: u32, height: u32, border: u32) -> Position {
    let mut rng = rand::thread_rng();

    Position {
        x: rng.gen_range(border as i32, (width - border) as i32),
        y: rng.gen_range(border as i32, (height - border) as i32),
    }
}

pub struct Game {
    snake: Snake,
    fruit: Position,
    size: (u32, u32),
    waiting_time: f64,
    score: u32,
    over: bool,
    paused: bool,
}

impl Game {
    pub fn new(width: u32, height: u32) -> Self {
        let border: u32 = (width as f64 * 0.25) as u32 + 3;

        let mut snake_pos: Position = calc_random_pos(width, height, border);
        let mut fruit_pos: Position = calc_random_pos(width, height, border);

        loop {
            if snake_pos.y != fruit_pos.y {
                break;
            }
        }

        Self {
            snake: Snake::new(calc_random_pos(width, height, border)),
            fruit: calc_random_pos(width, height, border),
            size: (width, height),
            waiting_time: 0.0,
            score: 0,
            over: false,
            paused: true,
        }
    }

    pub fn start(&mut self) {
        self.paused = false;
    }

    pub fn pause(&mut self) {
        self.paused = true;
    }

    pub fn toggle_game_state(&mut self) {
        if self.paused {
            self.start();
        } else {
            self.pause();
        }
    }

    pub fn draw(&self, ctx: Context, g: &mut G2d) {
        self.snake.draw(&ctx, g);
        draw::draw_block(&ctx, g, colors::FRUIT, &self.fruit);

        if self.over {
            rectangle(
                colors::OVERLAY,
                [
                    0.0,
                    0.0,
                    draw::blocks_in_pixels(self.size.0) as f64,
                    draw::blocks_in_pixels(self.size.1) as f64,
                ],
                ctx.transform,
                g,
            );
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        // if self.over {
        // if self.waiting_time > RESTART_TIME {
        //     self.restart();
        // }
        // return;
        // }

        if self.waiting_time > fps_in_ms(FPS) && !self.over && !self.paused {
            // self.check_colision() use snake.get_head_pos;
            self.waiting_time = 0.0;

            if self.snake.is_alive(self.size) {
                self.snake.update();
            } else {
                self.over = true;
            }
        }
    }

    pub fn key_down(&mut self, key: keyboard::Key) {
        use keyboard::Key;

        match key {
            Key::R => self.over = false, // temp solution -> replace current game state trough new one
            Key::Space => self.toggle_game_state(),
            _ => self.start(),
        }

        match key {
            Key::A | Key::Left => self.snake.set_dir(Direction::Left),
            Key::W | Key::Up => self.snake.set_dir(Direction::Up),
            Key::D | Key::Right => self.snake.set_dir(Direction::Right),
            Key::S | Key::Down => self.snake.set_dir(Direction::Down),
            _ => {}
        }
    }

    // fn update_snake(&mut self, dir: Option<Direction>) {
    //     if self.check_if_snake_alive(dir) {
    //         self.snake.move_forward(dir);
    //         self.check_eating();
    //     } else {
    //         self.game_over = true;
    //     }
    //     self.waiting_time = 0.0;
    // }
}
