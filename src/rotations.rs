use nalgebra::point;

use crate::Point;

pub fn _rot_x(p: Point, a: f64) -> Point {
    let s = a.sin();
    let c = a.cos();

    return point![p.x, p.y * c + p.z * s, -s * p.y + c * p.z];
}

pub fn _rot_y(p: Point, a: f64) -> Point {
    let s = a.sin();
    let c = a.cos();

    return point![c * p.x + s * p.z, p.y, -s * p.x + c * p.z];
}

pub fn _rot_z(p: Point, a: f64) -> Point {
    let s = a.sin();
    let c = a.cos();

    return point![c * p.x - s * p.y, s * p.x + c * p.y, p.z];
}
