// Main reference: https://iquilezles.org/articles/distfunctions/
use nalgebra::{distance, vector};

use crate::vector::vec3;
use crate::{Point, Vector};

pub fn _sphere(p: Point, c: Point, r: f64) -> f64 {
    return distance(&p, &c) - r;
}

pub fn _rounded_box(p: Point, s: Vector, r: f64) -> f64 {
    // Modified to account for the radius without changing the size of the box
    let pf = vector![p.x.abs(), p.y.abs(), p.z.abs()] - (s - vec3(r));
    return vector![pf.x.max(0.0), pf.y.max(0.0), pf.z.max(0.0)].norm()
        + pf.x.max(pf.y.max(pf.z)).min(0.0)
        - r;
}

pub fn _rounded_cylinder(p: Point, r1: f64, r2: f64, h: f64) -> f64 {
    let d = vector![vector![p.x, p.z].norm() - 2.0 * r1 + r2, p.y.abs() - h];
    return d.x.max(d.y).min(0.0) + vector![d.x.max(0.0), d.y.max(0.0)].norm() - r2;
}

pub fn _torus(p: Point, r1: f64, r2: f64) -> f64 {
    let q = vector![vector![p.x, p.z].norm() - r1, p.y];
    return q.norm() - r2;
}
