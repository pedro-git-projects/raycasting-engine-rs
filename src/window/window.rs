use std::f64::consts::PI;

use lazy_static::lazy_static;

pub const MINIMAP_SCALING: f64 = 0.2;
pub const TILE_SIZE: u32 = 64;
pub const NUM_ROWS: u32 = 600 / TILE_SIZE;
pub const NUM_COLS: u32 = 800 / TILE_SIZE;
pub const FOV: f64 = 60.0 * (PI / 180.0);
pub const WINDOW_WIDTH: u32 = NUM_COLS * TILE_SIZE;
pub const WINDOW_HEIGHT: u32 = NUM_ROWS * TILE_SIZE;
lazy_static! {
    pub static ref DISTANCE_PROJ_PLANE: f64 = (WINDOW_WIDTH as f64 / 2.0) / (FOV / 2.0).tan();
}
pub const NUM_RAYS: u32 = WINDOW_WIDTH;
