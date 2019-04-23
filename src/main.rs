mod draw;
mod fruit;
mod game;
mod physics;
mod snake;

use game::Game;
use piston_window::*;

fn main() {
    let (width, height) = (30, 30);

    let mut window: PistonWindow = WindowSettings::new("rsnake", [width * 25, height * 25])
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();

    let mut game = Game::new(width, height);

    while let Some(event) = window.next() {
        // if let Some(Button::Keyboard(key)) = event.press_args() {
        //     game.key_pressed(key);
        // }

        window.draw_2d(&event, |c, g| {
            clear([0.0, 0.0, 0.0, 1.0], g);
            // game.draw(&c, g);
        });

        //     event.update(|arg| {
        //         game.update(arg.dt);
        //     });
    }
}
