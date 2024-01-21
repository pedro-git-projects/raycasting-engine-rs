use crate::{
    ray::ray::Ray,
    utils::points::distance_between_points,
    window::window::{DISTANCE_PROJ_PLANE, NUM_RAYS, TILE_SIZE, WINDOW_HEIGHT, WINDOW_WIDTH},
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
    pub fn calculate_horizontal_intersection(
        &mut self,
        ray: &mut Ray,
    ) -> HorizontalIntersectionResult {
        let mut result = HorizontalIntersectionResult::default();

        let mut y_intersection = (self.player.y / TILE_SIZE as f64).floor() * TILE_SIZE as f64;
        if ray.is_facing_down {
            y_intersection += TILE_SIZE as f64;
        }

        let x_intersection = self.player.x + ((y_intersection - self.player.y) / ray.angle.tan());

        let mut y_step = TILE_SIZE as f64;
        if ray.is_facing_up {
            y_step *= -1.0;
        }

        let mut x_step = TILE_SIZE as f64 / ray.angle.tan();
        if ray.is_facing_left && x_step > 0.0 {
            x_step *= -1.0;
        }
        if ray.is_facing_right && x_step < 0.0 {
            x_step *= -1.0;
        }

        let mut next_horz_x_collision = x_intersection;
        let mut next_horz_y_collision = y_intersection;

        while next_horz_x_collision >= 0.0
            && next_horz_x_collision <= WINDOW_WIDTH as f64
            && next_horz_y_collision >= 0.0
            && next_horz_y_collision <= WINDOW_HEIGHT as f64
        {
            let x_to_check = next_horz_x_collision;
            let mut y_to_check = next_horz_y_collision;
            if ray.is_facing_up {
                y_to_check -= 1.0;
            }
            if self.game.is_coordinate_solid(x_to_check, y_to_check) {
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

    pub fn calculate_vertical_intersection(&mut self, ray: &mut Ray) -> VerticalIntersectionResult {
        let mut result = VerticalIntersectionResult::default();

        let mut x_intersection = (self.player.x / TILE_SIZE as f64).floor() * TILE_SIZE as f64;
        if ray.is_facing_right {
            x_intersection += TILE_SIZE as f64;
        }

        let y_intersection = self.player.y + ((x_intersection - self.player.x) * ray.angle.tan());

        let mut x_step = TILE_SIZE as f64;
        if ray.is_facing_left {
            x_step *= 1.0;
        }

        let mut y_step = TILE_SIZE as f64 * ray.angle.tan();
        if ray.is_facing_up && y_step > 0.0 {
            y_step *= -1.0;
        }
        if ray.is_facing_down && y_step < 0.0 {
            y_step *= -1.0;
        }

        let mut next_vert_x_collision = x_intersection;
        let mut next_vert_y_collision = y_intersection;

        while next_vert_x_collision >= 0.0
            && next_vert_x_collision <= WINDOW_WIDTH as f64
            && next_vert_y_collision >= 0.0
            && next_vert_y_collision <= WINDOW_HEIGHT as f64
        {
            let mut x_to_check = next_vert_x_collision;
            if ray.is_facing_left {
                x_to_check -= 1.0;
            }

            let y_to_check = next_vert_y_collision;
            if self.game.is_coordinate_solid(x_to_check, y_to_check) {
                result.vert_x_wall_collision = next_vert_x_collision;
                result.vert_y_wall_collision = next_vert_y_collision;
                result.vert_wall_content = self.game.game_map
                    [(y_to_check / TILE_SIZE as f64) as usize]
                    [(x_to_check / TILE_SIZE as f64) as usize];
                result.found_vert_collision = true;
                break;
            } else {
                next_vert_x_collision += x_step;
                next_vert_y_collision += y_step;
            }
        }
        result
    }

    pub fn cast_ray(&mut self, mut angle: f64, ray_id: i32) {
        self.game.rays[ray_id as usize] = Ray::new(&mut angle);
        let mut ray = self.game.rays[ray_id as usize];

        let h = self.calculate_horizontal_intersection(&mut ray);
        let v = self.calculate_vertical_intersection(&mut ray);

        let mut horz_collision_dist = f64::MAX;
        let mut vert_collision_dist = f64::MAX;

        if h.found_horz_collision {
            horz_collision_dist = distance_between_points(
                self.player.x,
                self.player.y,
                h.horz_x_wall_collision,
                h.horz_y_wall_collision,
            );
        }

        if v.found_vert_collision {
            vert_collision_dist = distance_between_points(
                self.player.x,
                self.player.y,
                v.vert_x_wall_collision,
                v.vert_y_wall_collision,
            );
        }

        if vert_collision_dist < horz_collision_dist {
            self.game.rays[ray_id as usize].distance = vert_collision_dist;
            self.game.rays[ray_id as usize].x_collision = v.vert_x_wall_collision;
            self.game.rays[ray_id as usize].y_collision = v.vert_y_wall_collision;
            self.game.rays[ray_id as usize].content = v.vert_wall_content;
            self.game.rays[ray_id as usize].is_vertical_collision = true;
        } else {
            self.game.rays[ray_id as usize].distance = horz_collision_dist;
            self.game.rays[ray_id as usize].x_collision = h.horz_x_wall_collision;
            self.game.rays[ray_id as usize].y_collision = h.horz_y_wall_collision;
            self.game.rays[ray_id as usize].content = h.horz_wall_content;
            self.game.rays[ray_id as usize].is_vertical_collision = false;
        }
    }

    pub fn cast_rays(&mut self) {
        for col in 0..NUM_RAYS {
            let angle = self.player.rotation_angle
                + ((col as f64 - NUM_RAYS as f64 / 2.0) / *DISTANCE_PROJ_PLANE as f64).atan();
            self.cast_ray(angle, col as i32);
        }
    }
}
