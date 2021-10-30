use crate::constants;
use crate::util::{Direction, Position};
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
	pub fn new(x: u32, y: u32) -> Snake {
		let mut body: VecDeque<Position> = VecDeque::new();
		for i in 0..constants::SNAKE_STARTING_LENGTH {
			body.push_back(Position::new(x - i, y));
		}
		Snake {
			body,
			dir: Direction::Right,
			last_dir: Direction::Right,
		}
	}

	pub fn update(&mut self) {
		let mut new_head = self.get_head().clone();
		match self.dir {
			Direction::Right => new_head.x += 1,
			Direction::Left => new_head.x -= 1,
			Direction::Down => new_head.y += 1,
			Direction::Up => new_head.y -= 1,
		}
		self.last_dir = self.dir.clone();
		self.body.push_front(new_head);
		self.body.pop_back();
	}

	pub fn get_head(&mut self) -> &mut Position {
		self.body.front_mut().unwrap()
	}

	pub fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
		let red: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
		let squares: Vec<Rectangle> = self
			.body
			.iter()
			.map(|position: &Position| {
				graphics::rectangle::square(
					(position.x * constants::PIXEL_SIZE as u32).into(),
					(position.y * constants::PIXEL_SIZE as u32).into(),
					constants::PIXEL_SIZE as f64,
				)
			})
			.collect();

		gl.draw(args.viewport(), |c, gl| {
			let transform = c.transform;
			squares.into_iter().for_each(|square| {
				graphics::rectangle(red, square, transform, gl);
			});
		});
	}

	pub fn pressed(&mut self, button: &Button) {
		self.dir = match button {
			&Button::Keyboard(Key::Up) if self.last_dir != Direction::Down => Direction::Up,
			&Button::Keyboard(Key::Down) if self.last_dir != Direction::Up => Direction::Down,
			&Button::Keyboard(Key::Left) if self.last_dir != Direction::Right => Direction::Left,
			&Button::Keyboard(Key::Right) if self.last_dir != Direction::Left => Direction::Right,
			&_ => self.last_dir.clone(),
		}
	}
}
