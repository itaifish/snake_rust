#[derive(Clone, PartialEq)]
pub enum Direction {
	Left,
	Right,
	Up,
	Down,
}

#[derive(Clone)]
pub struct Position {
	pub x: u32,
	pub y: u32,
}

impl Position {
	pub fn new(x: u32, y: u32) -> Position {
		Position { x, y }
	}
}
