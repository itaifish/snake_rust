use crate::constants;
use crate::util::{Direction, Position, DIRECTION_KEYS};
use graphics::types::Rectangle;
use opengl_graphics::GlGraphics;
use piston::input::*;
use std::collections::VecDeque;

pub struct Snake {
	pub body: VecDeque<Position>,
	pub dir: Direction,
	pub last_dir: Direction,
}

impl Snake {
	pub fn new(x: i32, y: i32) -> Snake {
		let mut body: VecDeque<Position> = VecDeque::new();
		for i in 0..constants::SNAKE_STARTING_LENGTH {
			body.push_back(Position::new(x - i as i32, y));
		}
		Snake {
			body,
			dir: Direction::Right,
			last_dir: Direction::Right,
		}
	}

	pub fn update(&mut self) {
		let mut new_head = *self.head_mut();
		match self.dir {
			Direction::Right => new_head.x += 1,
			Direction::Left => new_head.x -= 1,
			Direction::Down => new_head.y += 1,
			Direction::Up => new_head.y -= 1,
		}
		self.last_dir = self.dir;
		self.body.push_front(new_head);
		self.body.pop_back();
	}

	pub fn head_mut(&mut self) -> &mut Position {
		self.body.front_mut().unwrap()
	}

	pub fn grow(&mut self) {
		self.body.push_back(*self.body.back().unwrap());
	}

	pub fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
		let green: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
		let dark_green: [f32; 4] = [0.0, 0.5, 0.0, 1.0];
		let squares: Vec<Rectangle> = self
			.body
			.iter()
			.map(|position: &Position| {
				graphics::rectangle::square(
					(position.x * constants::PIXEL_SIZE as i32).into(),
					(position.y * constants::PIXEL_SIZE as i32).into(),
					constants::PIXEL_SIZE as f64,
				)
			})
			.collect();

		gl.draw(args.viewport(), |c, gl| {
			let transform = c.transform;
			let mut iter = squares.into_iter();
			let head = iter.next().unwrap();
			graphics::rectangle(dark_green, head, transform, gl);
			iter.for_each(|square| {
				graphics::rectangle(green, square, transform, gl);
			});
		});
	}

	pub fn pressed(&mut self, button: &Button) {
		let dir = DIRECTION_KEYS.get(button).unwrap_or(&self.last_dir);
		self.dir = {
			if self.last_dir != dir.opposite() {
				*dir
			} else {
				self.last_dir
			}
		};
	}
}
