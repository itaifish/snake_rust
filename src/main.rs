extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

mod constants;
mod game;
mod snake;
mod util;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

fn main() {
    let opengl = OpenGL::V4_5;
    let mut window: GlutinWindow = WindowSettings::new("Snake Game", constants::GAME_SIZE)
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = game::Game::new(GlGraphics::new(opengl));
    let mut events = Events::new(EventSettings::new()).ups(constants::UPDATES_PER_SECOND);
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            game.render(&r);
        }
        if let Some(_u) = e.update_args() {
            game.update();
        }
        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                game.pressed(&k.button);
            }
        }
    }
}
