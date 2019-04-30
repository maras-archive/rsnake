mod colors;
mod draw;
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

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    let ref font = assets.join("retro-gaming.ttf");
    let factory = window.factory.clone();
    let mut glyphs = Glyphs::new(font, factory, TextureSettings::new()).unwrap();

    let mut main: Game = Game::new(WIDTH, HEIGHT);
    main.start();

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            main.key_down(key);
        }

        window.draw_2d(&event, |ctx, g| {
            clear(colors::BACKGROUND, g);
            text::Text::new_color([1.0, 1.0, 1.0, 1.0], 20)
                .draw(
                    main.get_score().to_string().as_ref(),
                    &mut glyphs,
                    &ctx.draw_state,
                    ctx.transform.trans(0.0, 20.0),
                    g,
                )
                .unwrap();
            main.draw(ctx, g);
        });

        event.update(|arg| {
            main.update(arg.dt);
        });
    }
}
