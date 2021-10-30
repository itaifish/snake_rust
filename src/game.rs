use crate::constants;
use crate::snake;

use opengl_graphics::GlGraphics;
use piston::input::*;

pub struct Game {
	gl: GlGraphics,
	snake: snake::Snake,
}

impl Game {
	pub fn new(gl_graphics: GlGraphics) -> Game {
		let mid_point = [
			constants::GAME_SIZE_NUM_PIXELS[0] / 2,
			constants::GAME_SIZE_NUM_PIXELS[1] / 2,
		];
		Game {
			gl: gl_graphics,
			snake: snake::Snake::new(mid_point[0], mid_point[1]),
		}
	}

	pub fn render(&mut self, args: &RenderArgs) {
		let red: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
		self.gl.draw(args.viewport(), |_c, gl| {
			graphics::clear(red, gl);
		});
		self.snake.render(&mut self.gl, args);
	}

	pub fn update(&mut self) {
		self.snake.update();
	}

	pub fn pressed(&mut self, button: &Button) {
		self.snake.pressed(button);
	}
}
