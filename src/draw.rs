extern crate piston_window;

use piston_window::{types::Color, *};

const BACKGROUND: Color = [0.0, 0.0, 0.0, 1.0];

pub fn clear_window(window: &mut PistonWindow, e: Event) {
    window.draw_2d(&e, |_c, g| {
        clear(BACKGROUND, g);
    });
}
