use crate::utils::geometry::{is_angle_facing_down, is_angle_facing_right, normalize_angle};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub angle: f64,
    pub x_collision: f64,
    pub y_collision: f64,
    pub distance: f64,
    pub is_vertical_collision: bool,
    pub is_facing_up: bool,
    pub is_facing_down: bool,
    pub is_facing_left: bool,
    pub is_facing_right: bool,
    pub content: i32,
}

impl Ray {
    pub fn new(mut angle: &mut f64) -> Ray {
        normalize_angle(&mut angle);
        let is_facing_down = is_angle_facing_down(&mut angle);
        let is_facing_up = !is_facing_down;
        let is_facing_right = is_angle_facing_right(&mut angle);
        let is_facing_left = !is_facing_right;

        Ray {
            angle: *angle,
            x_collision: f64::default(),
            y_collision: f64::default(),
            distance: f64::default(),
            is_vertical_collision: false,
            is_facing_up,
            is_facing_down,
            is_facing_left,
            is_facing_right,
            content: i32::default(),
        }
    }
}
