extern crate piston_window;
extern crate rand;

mod draw;
mod fruit;
mod game;
mod snake;

use piston_window::*;

const BACKGROUND: types::Color = [0.0, 0.0, 0.0, 1.0];

fn main() {
    let state = game::State::new();
    let mut window: PistonWindow = WindowSettings::new("rsnake", [640, 640])
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();

    while let Some(e) = window.next() {
        window.draw_2d(&e, |_c, g| {
            clear(BACKGROUND, g);
        });
    }
}
