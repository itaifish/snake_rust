use crate::constants;
use crate::snake;
use crate::util::Position;
use rand::Rng;

use opengl_graphics::GlGraphics;
use piston::input::{Button, RenderArgs};
use std::{thread, time};

pub struct Game {
	gl: GlGraphics,
	snake: snake::Snake,
	apple: Position,
	score: u32,
}

impl Game {
	pub fn new(gl_graphics: GlGraphics) -> Game {
		let mid_point = [
			constants::GAME_SIZE_NUM_PIXELS[0] / 2,
			constants::GAME_SIZE_NUM_PIXELS[1] / 2,
		];
		Game {
			gl: gl_graphics,
			snake: snake::Snake::new(mid_point[0] as i32, mid_point[1] as i32),
			apple: Game::random_position(),
			score: 0,
		}
	}

	pub fn render(&mut self, args: &RenderArgs) {
		let background: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
		self.gl.draw(args.viewport(), |_c, gl| {
			graphics::clear(background, gl);
		});
		let square = graphics::rectangle::square(
			(self.apple.x * constants::PIXEL_SIZE as i32).into(),
			(self.apple.y * constants::PIXEL_SIZE as i32).into(),
			constants::PIXEL_SIZE.into(),
		);
		let red: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
		self.gl.draw(args.viewport(), |c, gl| {
			let transform = c.transform;
			graphics::rectangle(red, square, transform, gl);
		});
		self.snake.render(&mut self.gl, args);
	}

	pub fn update(&mut self) {
		self.snake.update();
		self.check_collision();
	}

	pub fn pressed(&mut self, button: &Button) {
		self.snake.pressed(button);
	}

	fn random_position() -> Position {
		let mut rng = rand::thread_rng();
		let x = rng.gen_range(0..constants::GAME_SIZE_NUM_PIXELS[0]) as i32;
		let y = rng.gen_range(0..constants::GAME_SIZE_NUM_PIXELS[1]) as i32;
		Position { x, y }
	}

	fn reset(&mut self) {
		println!("Died with score of: {}", self.score);
		let half_second = time::Duration::from_millis(500);
		thread::sleep(half_second);
		let mid_point = [
			constants::GAME_SIZE_NUM_PIXELS[0] / 2,
			constants::GAME_SIZE_NUM_PIXELS[1] / 2,
		];
		self.apple = Game::random_position();
		self.snake = snake::Snake::new(mid_point[0] as i32, mid_point[1] as i32);
		self.score = 0;
	}

	fn check_collision(&mut self) {
		// apple collision
		let head_position = *self.snake.head_mut();
		if head_position == self.apple {
			self.snake.grow();
			self.score += 1;
			self.apple = Game::random_position();
			return;
		}
		// self collision
		for i in 1..self.snake.body.len() {
			let pos = self.snake.body[i];
			if head_position == pos {
				println!("Oops, hit myself");
				self.reset();
				return;
			}
		}
		// oob
		if head_position.x < 0
			|| head_position.y < 0
			|| head_position.x >= constants::GAME_SIZE_NUM_PIXELS[0] as i32
			|| head_position.y >= constants::GAME_SIZE_NUM_PIXELS[1] as i32
		{
			println!("Out of bounds");
			self.reset();
		}
	}
}
