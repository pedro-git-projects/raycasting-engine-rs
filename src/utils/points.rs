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
