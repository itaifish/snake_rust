use lazy_static::lazy_static;
use piston::input::{Button, Key};
use std::collections::HashMap;

#[derive(Clone, PartialEq, Copy)]
pub enum Direction {
	Left,
	Right,
	Up,
	Down,
}

impl Direction {
	pub fn opposite(&self) -> Direction {
		match *self {
			Direction::Up => Direction::Down,
			Direction::Down => Direction::Up,
			Direction::Left => Direction::Right,
			Direction::Right => Direction::Left,
		}
	}
}

lazy_static! {
	pub static ref DIRECTION_KEYS: HashMap<Button, Direction> = {
		let mut m = HashMap::new();
		m.insert(Button::Keyboard(Key::Up), Direction::Up);
		m.insert(Button::Keyboard(Key::W), Direction::Up);
		m.insert(Button::Keyboard(Key::Down), Direction::Down);
		m.insert(Button::Keyboard(Key::S), Direction::Down);
		m.insert(Button::Keyboard(Key::Left), Direction::Left);
		m.insert(Button::Keyboard(Key::A), Direction::Left);
		m.insert(Button::Keyboard(Key::Right), Direction::Right);
		m.insert(Button::Keyboard(Key::D), Direction::Right);

		m
	};
}

#[derive(Clone, Eq, PartialEq, Copy)]
pub struct Position {
	pub x: i32,
	pub y: i32,
}

impl Position {
	pub fn new(x: i32, y: i32) -> Position {
		Position { x, y }
	}
}
