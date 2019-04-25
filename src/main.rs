mod colors;
mod draw;
mod fruit;
mod game;
mod physics;
mod snake;

use draw::blocks_in_pixels;
use game::Game;
use piston_window::*;

const WINDOW_TITLE: &'static str = "rsnake";
const WIDTH: u32 = 25;
const HEIGHT: u32 = 25;

fn main() {
    let size = [blocks_in_pixels(WIDTH), blocks_in_pixels(HEIGHT)];

    let mut window: PistonWindow = WindowSettings::new(WINDOW_TITLE, size)
        .resizable(false)
        .build()
        .unwrap();

    let mut main: Game = Game::new(WIDTH, HEIGHT);

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            main.key_down(key);
        }

        window.draw_2d(&event, |ctx, g| {
            clear(colors::BACKGROUND, g);
            main.draw(ctx, g);
        });

        event.update(|arg| {
            main.update(arg.dt);
        });
    }
}
