use crate::{
    ray::ray::Ray,
    window::window::{TILE_SIZE, WINDOW_HEIGHT, WINDOW_WIDTH},
};

use super::app::App;

#[derive(Default)]
pub struct VerticalIntersectionResult {
    pub vert_x_wall_collision: f64,
    pub vert_y_wall_collision: f64,
    pub vert_wall_content: i32,
    pub found_vert_collision: bool,
}

#[derive(Default)]
pub struct HorizontalIntersectionResult {
    pub horz_x_wall_collision: f64,
    pub horz_y_wall_collision: f64,
    pub horz_wall_content: i32,
    pub found_horz_collision: bool,
}

impl<'a> App<'a> {
    fn is_solid_coordinate(&self, x: f64, y: f64) -> bool {
        if x < 0.0 || x >= WINDOW_WIDTH as f64 || y < 0.0 || y >= WINDOW_HEIGHT as f64 {
            return true;
        }

        let ind_x = (x / TILE_SIZE as f64).floor() as usize;
        let ind_y = (y / TILE_SIZE as f64).floor() as usize;
        self.game.game_map[ind_y][ind_x] != 0
    }

    pub fn calculate_vertical_intersection(&self, ray: &Ray) -> VerticalIntersectionResult {
        let mut result = VerticalIntersectionResult {
            ..Default::default()
        };

        let x_intersection = (self.player.x / TILE_SIZE as f64).floor() * TILE_SIZE as f64;
        let x_intersection = if Ray::is_facing_right(ray.angle) {
            x_intersection + TILE_SIZE as f64
        } else {
            x_intersection
        };

        let y_intersection = self.player.y + (x_intersection - self.player.x) * ray.angle.tan();

        let x_step = TILE_SIZE as f64
            * if !Ray::is_facing_right(ray.angle) {
                -1.0
            } else {
                1.0
            };

        let y_step = TILE_SIZE as f64
            * ray.angle.tan()
            * if !Ray::is_facing_down(ray.angle) || Ray::is_facing_down(ray.angle) {
                -1.0
            } else {
                1.0
            };

        let mut next_vert_x_collision = x_intersection;
        let mut next_vert_y_collision = y_intersection;

        while next_vert_x_collision >= 0.0
            && next_vert_x_collision <= WINDOW_WIDTH as f64
            && next_vert_y_collision >= 0.0
            && next_vert_y_collision <= WINDOW_WIDTH as f64
        {
            let x_to_check = if !Ray::is_facing_right(ray.angle) {
                next_vert_x_collision - 1.0
            } else {
                next_vert_x_collision
            };
            let y_to_check = next_vert_y_collision;

            if self.is_solid_coordinate(x_to_check, y_to_check) {
                result.vert_x_wall_collision = next_vert_x_collision;
                result.vert_y_wall_collision = next_vert_y_collision;
                result.vert_wall_content = self.game.game_map
                    [(y_to_check / TILE_SIZE as f64).floor() as usize]
                    [(x_to_check / TILE_SIZE as f64).floor() as usize];
                result.found_vert_collision = true;
                break;
            } else {
                next_vert_x_collision += x_step;
                next_vert_y_collision += y_step;
            }
        }

        result
    }

    // if problemas arise check the !Ray::is_facing_x
    pub fn calculate_horizontal_intersection(&self, ray: &Ray) -> HorizontalIntersectionResult {
        let mut result = HorizontalIntersectionResult {
            ..Default::default()
        };

        let y_intersection = (self.player.y / TILE_SIZE as f64).floor() * TILE_SIZE as f64;
        let y_intersection = if Ray::is_facing_down(ray.angle) {
            y_intersection + TILE_SIZE as f64
        } else {
            y_intersection
        };

        let x_intersection = self.player.x + (y_intersection - self.player.y) / ray.angle.tan();

        let y_step = TILE_SIZE as f64
            * if !Ray::is_facing_down(ray.angle) {
                -1.0
            } else {
                1.0
            };

        let x_step = TILE_SIZE as f64 / ray.angle.tan()
            * if !Ray::is_facing_right(ray.angle) || Ray::is_facing_right(ray.angle) {
                -1.0
            } else {
                1.0
            };
        println!("xstep {}", x_step);

        let mut next_horz_x_collision = x_intersection;
        let mut next_horz_y_collision = y_intersection;

        while next_horz_x_collision >= 0.0
            && next_horz_x_collision <= WINDOW_WIDTH as f64
            && next_horz_y_collision >= 0.0
            && next_horz_y_collision <= WINDOW_HEIGHT as f64
        {
            let x_to_check = next_horz_x_collision;
            let y_to_check = if !Ray::is_facing_down(ray.angle) {
                next_horz_y_collision - 1.0
            } else {
                next_horz_y_collision
            };

            if self.is_solid_coordinate(x_to_check, y_to_check) {
                result.horz_x_wall_collision = next_horz_x_collision;
                result.horz_y_wall_collision = next_horz_y_collision;
                result.horz_wall_content = self.game.game_map
                    [(y_to_check / TILE_SIZE as f64).floor() as usize]
                    [(x_to_check / TILE_SIZE as f64).floor() as usize];
                result.found_horz_collision = true;
                break;
            } else {
                next_horz_x_collision += x_step;
                next_horz_y_collision += y_step;
            }
        }

        result
    }
}
