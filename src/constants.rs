pub const PIXEL_SIZE: u8 = 15;
pub const GAME_SIZE_NUM_PIXELS: [u32; 2] = [40, 50];
pub const GAME_SIZE: [u32; 2] = [
	GAME_SIZE_NUM_PIXELS[0] * PIXEL_SIZE as u32,
	GAME_SIZE_NUM_PIXELS[1] * PIXEL_SIZE as u32,
];
pub const UPDATES_PER_SECOND: u64 = 15;
pub const SNAKE_STARTING_LENGTH: u32 = 5;
