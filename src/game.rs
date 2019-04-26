use piston_window::*;

use crate::{
    colors, draw,
    fruit::Fruit,
    physics::{Direction, Position},
    snake::Snake,
};

const FPS: f64 = 8.0;
// const RESTART_TIME: f64 = 1.0;

fn fps_in_ms(fps: f64) -> f64 {
    1.0 / fps
}

pub struct Game {
    snake: Snake,
    fruit: Fruit,
    size: (u32, u32),
    waiting_time: f64,
    score: u32,
    over: bool,
    paused: bool,
}

impl Game {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            snake: Snake::new((width / 2) as i32, 4),
            fruit: Fruit::new(),
            size: (width, height),
            waiting_time: 0.0,
            score: 0,
            over: false,
            paused: false,
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

        // if !self.food_exists {
        //     self.add_food();
        // }

        if self.waiting_time > fps_in_ms(FPS) && !self.over {
            // self.check_colision() use snake.get_head_pos;
            self.waiting_time = 0.0;

            if self.snake.is_alive(self.size) {
                self.snake.update();
            } else {
                self.over = true;
            }
        }
    }

    pub fn key_down(&mut self, k: keyboard::Key) {
        use keyboard::Key;

        match k {
            Key::A | Key::Left => self.snake.set_dir(Direction::Left),
            Key::W | Key::Up => self.snake.set_dir(Direction::Up),
            Key::D | Key::Right => self.snake.set_dir(Direction::Right),
            Key::S | Key::Down => self.snake.set_dir(Direction::Down),
            Key::R => self.over = false, // temp solution -> replace current game state trough new one
            Key::Space => self.toggle_game_state(),
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
