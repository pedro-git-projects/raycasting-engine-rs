use std::f64::consts::PI;

pub const MINIMAP_SCALING: f64 = 0.2;
pub const TILE_SIZE: u32 = 64;
pub const NUM_ROWS: u32 = 600 / TILE_SIZE;
pub const NUM_COLS: u32 = 800 / TILE_SIZE;
pub const FOV: f64 = 60.0 * (std::f64::consts::PI / 180.0);

pub fn calculate_width() -> u32 {
    NUM_COLS * TILE_SIZE
}

pub fn calculate_height() -> u32 {
    NUM_ROWS * TILE_SIZE
}

pub fn calculate_distance_proj_plane(width: f64) -> f64 {
    (width / 2.0) / (FOV / 2.0).tan()
}

pub fn calculate_num_rays(width: u32) -> u32 {
    width
}
