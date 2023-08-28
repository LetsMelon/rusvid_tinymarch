use crate::math::{clamp, lerp};

pub fn _boolean_union(d1: f64, d2: f64) -> f64 {
    return d2.min(d1);
}

pub fn _boolean_subtraction(d1: f64, d2: f64) -> f64 {
    return (-d2).max(d1);
}

pub fn _boolean_intersection(d1: f64, d2: f64) -> f64 {
    return d1.max(d2);
}

pub fn _smooth_union(d1: f64, d2: f64, k: f64) -> f64 {
    let h = clamp(0.5 + 0.5 * (d2 - d1) / k, 0.0, 1.0);
    return lerp(d2, d1, h) - k * h * (1.0 - h);
}

pub fn _smooth_subtraction(d1: f64, d2: f64, k: f64) -> f64 {
    let h = clamp(0.5 - 0.5 * (d2 + d1) / k, 0.0, 1.0);
    return lerp(d1, -d2, h) + k * h * (1.0 - h);
}

pub fn _smooth_intersection(d1: f64, d2: f64, k: f64) -> f64 {
    let h = clamp(0.5 - 0.5 * (d2 - d1) / k, 0.0, 1.0);
    return lerp(d2, d1, h) + k * h * (1.0 - h);
}
