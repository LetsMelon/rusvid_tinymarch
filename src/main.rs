use std::time::Instant;

use nalgebra::{point, vector};
use tinymarch::math::lerp;
use tinymarch::sdf_operations::_smooth_subtraction;
use tinymarch::signed_distance_fields::_sphere;
use tinymarch::vector::{reflect, vec3};
use tinymarch::{gradient, render, Color, Point, Vector};

// Environment
fn sky(uv: (f64, f64)) -> Color {
    // Background
    let mut c = vector![0.1, 0.7, 1.];

    c += vec3(lerp(0.2, 0.4, 1.0 - uv.1));
    c += vec3(lerp(0.2, 0.4, uv.0));

    return c;
}

fn eval(p: Point) -> f64 {
    let s1 = _sphere(p, point![0.0, 0.0, 0.0], 1.);
    let s2 = _sphere(p, point![1., -0.6, -1.], 0.9);
    // return _boolean_union(s1, s2);
    return _smooth_subtraction(s1, s2, 0.05);
    // return _smooth_intersection(s1, s2, 0.5);
}

fn simple_shading(p: Point, rd: Vector, eval: fn(Point) -> f64) -> Color {
    let n = gradient(p, eval);
    let mut color = vector![0.2, 0.8, 1.];

    // lighting
    let light1 = n.dot(&vector![1., -1., -1.].normalize()) * 0.5 + 0.5;
    let light2 = n.dot(&vector![-1., -1., -1.].normalize()) * 0.5 + 0.5;
    let illumination = 0.5 * light1 + 0.5 * light2;

    // fake fresnel
    let n_dot_v = n.dot(&rd) + 1.;
    let fresnel = n_dot_v * n_dot_v * 0.45;

    // Specular highlights
    let r = reflect(vector![1., 0., 0.].normalize(), n);
    let specular = vec3(1.0) * r.dot(&rd).max(0.0).powf(10.0) * 0.08;

    color *= illumination;
    color += vec3(fresnel);
    color += specular;

    return color;
}

fn main() {
    let now = Instant::now();

    let x_res = 1080;
    let y_res = 1080;
    let sample_count = 10; // anti-aliasing

    render(x_res, y_res, sample_count, eval, sky, simple_shading);

    let elapsed = now.elapsed().as_secs_f64();
    let s = elapsed % 60.;
    let min = (elapsed / 60.).floor() as u8;
    println!("\n{} min {:.2?} seconds", min, s);
}
