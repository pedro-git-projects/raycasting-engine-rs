use std::f64::consts::PI;

pub fn distance_between_points(x0: f64, y0: f64, x1: f64, y1: f64) -> f64 {
    ((x1 - x0) * (x1 - x0) + (y1 - y0) * (y1 - y0)).sqrt()
}

pub fn normalize_angle(angle: &mut f64) {
    *angle = angle.rem_euclid(2.0 * PI);
    if *angle < 0.0 {
        *angle += 2.0 * PI;
    }
}

pub fn is_angle_facing_down(angle: &mut f64) -> bool {
    *angle > 0.0 && *angle < PI
}

pub fn is_angle_facing_right(angle: &mut f64) -> bool {
    *angle < 0.5 * PI || *angle > 1.5 * PI
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_between_points() {
        assert_eq!(distance_between_points(0.0, 0.0, 0.0, 0.0), 0.0);
        assert_eq!(distance_between_points(0.0, 0.0, 3.0, 4.0), 5.0);
        assert_eq!(distance_between_points(1.0, 2.0, 4.0, 6.0), 5.0);
    }

    #[test]
    fn test_normalize_angle() {
        // positive angle normalization
        let mut angle = 3.0 * PI;
        normalize_angle(&mut angle);
        assert_eq!(angle, PI);

        // negative angle normalization
        let mut angle = -PI;
        normalize_angle(&mut angle);
        assert_eq!(angle, PI);

        // angle within the range
        let mut angle = 0.5 * PI;
        normalize_angle(&mut angle);
        assert_eq!(angle, 0.5 * PI);
    }

    #[test]
    fn test_is_angle_facing_down() {
        // angle facing down
        let mut angle = 0.25 * PI;
        assert!(is_angle_facing_down(&mut angle));

        // angle not facing down
        let mut angle = 1.5 * PI;
        assert!(!is_angle_facing_down(&mut angle));

        // angle at the boundary
        let mut angle = PI;
        assert!(!is_angle_facing_down(&mut angle));
    }

    #[test]
    fn test_is_angle_facing_right() {
        // angle facing right
        let mut angle = 0.25 * PI;
        assert!(is_angle_facing_right(&mut angle));

        // angle not facing right
        let mut angle = 1.5 * PI;
        assert!(!is_angle_facing_right(&mut angle));

        // angle at the boundary
        let mut angle = PI;
        assert!(!is_angle_facing_right(&mut angle));
    }
}
