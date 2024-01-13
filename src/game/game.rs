use crate::{
    ray::ray::Ray,
    window::window::{NUM_RAYS, TILE_SIZE, WINDOW_HEIGHT, WINDOW_WIDTH},
};

pub struct Game {
    pub game_map: [[i32; 20]; 13],
    rays: Vec<Ray>,
}

impl Default for Game {
    fn default() -> Self {
        let rays: Vec<Ray> = vec![Ray::new(0.0); NUM_RAYS as usize];

        Game {
            game_map: Self::initialize_game_map(),
            rays,
        }
    }
}

impl Game {
    fn initialize_game_map() -> [[i32; 20]; 13] {
        [
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 2, 2, 0, 3, 0, 4, 0, 5, 0, 6, 0, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 5],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 5],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 5],
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 5, 5, 5, 5, 5, 5],
        ]
    }

    pub fn new() -> Self {
        let rays: Vec<Ray> = vec![Ray::new(0.0); NUM_RAYS as usize];

        Game {
            game_map: Self::initialize_game_map(),
            rays,
        }
    }

    pub fn is_coordinate_solid(&self, x: f64, y: f64) -> bool {
        if x < 0.0 || x >= WINDOW_WIDTH as f64 || y < 0.0 || y > WINDOW_HEIGHT as f64 {
            return true;
        }
        let ind_x = (x / TILE_SIZE as f64).floor() as usize;
        let ind_y = (y / TILE_SIZE as f64).floor() as usize;

        self.game_map[ind_y][ind_x] != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_coordinate_solid_outside_window() {
        let game = Game::new();
        assert!(game.is_coordinate_solid(-1.0, 10.0));
        assert!(game.is_coordinate_solid(10.0, 10.0));
        assert!(game.is_coordinate_solid(5.0, -1.0));
        assert!(game.is_coordinate_solid(5.0, 20.0));
    }

    #[test]
    fn test_is_coordinate_solid_empty_space() {
        let game = Game::new();
        assert!(!game.is_coordinate_solid(2.0 * TILE_SIZE as f64, 2.0 * TILE_SIZE as f64));
        assert!(!game.is_coordinate_solid(6.0 * TILE_SIZE as f64, 6.0 * TILE_SIZE as f64));
    }

    #[test]
    fn test_is_coordinate_solid_obstacle() {
        let game = Game::new();
        assert!(game.is_coordinate_solid(11.0 * TILE_SIZE as f64, 12.0 * TILE_SIZE as f64));
    }
}
