pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + t * (b - a)
}

pub fn clamp(x: f64, a: f64, b: f64) -> f64 {
    x.max(a).min(b)
}
