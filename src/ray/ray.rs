use std::f64::consts::PI;

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
    pub fn is_facing_down(angle: f64) -> bool {
        angle > 0.0 && angle < PI
    }

    pub fn is_facing_right(angle: f64) -> bool {
        angle < 0.5 * PI || angle > 1.5 * PI
    }

    pub fn normalize_angle(angle: f64) -> f64 {
        let mut angle = angle.rem_euclid(2.0 * PI);
        if angle < 0.0 {
            angle += 2.0 * PI
        }
        angle
    }

    pub fn new(angle: f64) -> Ray {
        let angle = Self::normalize_angle(angle);
        let is_facing_down = Self::is_facing_down(angle);
        let is_facing_up = !is_facing_down;
        let is_facing_right = Self::is_facing_right(angle);
        let is_facing_left = !is_facing_right;

        Ray {
            angle,
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

    pub fn set_distance(&mut self, distance: f64) {
        self.distance = distance;
    }

    pub fn set_x_collision(&mut self, x_collision: f64) {
        self.x_collision = x_collision;
    }

    pub fn set_y_collision(&mut self, y_collision: f64) {
        self.y_collision = y_collision;
    }

    pub fn set_content(&mut self, content: i32) {
        self.content = content;
    }

    pub fn set_is_vertical_collision(&mut self, is_vertical_collision: bool) {
        self.is_vertical_collision = is_vertical_collision;
    }
}

#[test]
fn test_normalize_angle_positive() {
    assert_eq!(Ray::normalize_angle(1.5 * PI), 1.5 * PI);
}

#[test]
fn test_normalize_angle_negative() {
    assert_eq!(Ray::normalize_angle(-0.5 * PI), 1.5 * PI);
}
