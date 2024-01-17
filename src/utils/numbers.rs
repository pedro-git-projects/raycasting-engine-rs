pub fn wrapping_sub_float(a: f64, b: f64) -> f64 {
    if a > b {
        a - b
    } else {
        f64::MIN
    }
}
