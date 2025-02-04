use std::f64::consts::PI;

use image::{Rgb, RgbImage};
use nalgebra::{point, vector, Point3, Vector3};
use rand::Rng;
use rayon::prelude::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

pub mod math;
pub mod rotations;
pub mod sdf_operations;
pub mod signed_distance_fields;
pub mod vector;

use crate::vector::vec3;

// ======================================================================
// ======================= Data types & Constants =======================
// ======================================================================

pub type Point = Point3<f64>;
pub type Vector = Vector3<f64>;
pub type Color = Vector3<f64>;

pub const MAX_STEPS: usize = 500;
pub const MAX_DIST: f64 = 100.;
pub const SURF_DIST: f64 = 0.0001;
pub const TAU: f64 = PI * 2.0;
pub const STEP_SCALE: f64 = 1.0;

pub const BLACK: Color = vector![0.0, 0.0, 0.0];
pub const WHITE: Color = vector![1.0, 1.0, 1.0];
pub const RED: Color = vector![1.0, 0.0, 0.0];
pub const GREEN: Color = vector![0.0, 1.0, 0.0];
pub const BLUE: Color = vector![0.0, 0.0, 1.0];

// ======================================================================
// ==================== Main Loop & Render Functions ====================
// ======================================================================

pub fn render(
    res_x: usize,
    res_y: usize,
    samples: usize,
    eval: fn(Point) -> f64,
    sky: fn((f64, f64)) -> Color,
    simple_shading: fn(p: Point, rd: Vector, eval: fn(Point) -> f64) -> Color,
) {
    let ro = point![0., 0., -10.];
    let rt = point![0., 0., 0.];

    // sampling
    let sample_scale = 1. / (samples as f64);

    let pixels = (0..res_x)
        .into_par_iter()
        .map(|x| {
            (0..res_y)
                .into_par_iter()
                .rev()
                .map(|y| {
                    let mut color = vec3(0.0);
                    let mut sampler = rand::thread_rng();
                    for _ in 0..samples {
                        // screen space coordinates
                        let u = (x as f64 + sampler.gen::<f64>()) / res_x as f64;
                        let v = 1.0 - (y as f64 + sampler.gen::<f64>()) / res_y as f64; // flip y

                        // ray direction
                        let rd = ray_direction((u - 0.5, v - 0.5), ro, rt, (res_x, res_y));

                        // ray marching
                        let d = ray_march(ro, rd, eval);

                        // shading
                        if d >= MAX_DIST {
                            color += sky((u, v));
                        } else {
                            let p = ro + d * rd;
                            color += simple_shading(p, rd, eval);
                        }
                    }
                    color *= sample_scale;
                    color
                })
                .collect()
        })
        .collect();

    save_png(pixels, "output.png")
}

// Ray direction
fn ray_direction(uv: (f64, f64), ro: Point, rt: Point, res: (usize, usize)) -> Vector {
    // screen orientation
    let vup = vector![0., 1.0, 0.0];
    let aspect_ratio = (res.0 as f64) / (res.1 as f64);

    let vw = (ro - rt).normalize();
    let vu = (vup.cross(&vw)).normalize();
    let vv = vw.cross(&vu);
    let theta = 30. * 2. * PI / 180.; // half FOV
    let viewport_height = 2. * (theta.tan());
    let viewport_width = aspect_ratio * viewport_height;
    let horizontal = -viewport_width * vu;
    let vertical = viewport_height * vv;
    let focus_dist = (ro - rt).norm();
    let center = ro - vw * focus_dist;

    let rd = center + uv.0 * horizontal + uv.1 * vertical - ro;

    return rd.normalize();
}

pub fn ray_march(ro: Point, rd: Vector, eval: fn(Point) -> f64) -> f64 {
    let mut d = 0.0;

    for _ in 0..MAX_STEPS {
        let p = ro + rd * d;
        let ds = eval(p);
        d += ds * STEP_SCALE;
        if d >= MAX_DIST || ds < SURF_DIST {
            break;
        }
    }
    return d;
}

////////////////////////////////////////////////////////////////
// Signed Distance Fields
////////////////////////////////////////////////////////////////

// Gradient of a Signed Distance Field
pub fn gradient(p: Point, eval: fn(Point) -> f64) -> Vector {
    let epsilon = 0.0001;
    let dx = Vector3::new(epsilon, 0., 0.);
    let dy = Vector3::new(0., epsilon, 0.);
    let dz = Vector3::new(0., 0., epsilon);

    // Gradient: dSDF/dx, dy, dz
    let ddx = eval(p + dx) - eval(p - dx);
    let ddy = eval(p + dy) - eval(p - dy);
    let ddz = eval(p + dz) - eval(p - dz);

    vector![ddx, ddy, ddz].normalize()
}

////////////////////////////////////////////////////////////////
// Image Writing
////////////////////////////////////////////////////////////////

pub fn save_png(pixels: Vec<Vec<Color>>, path: &str) {
    let width = pixels.len() as u32;
    let height = pixels[0].len() as u32;

    let mut img = RgbImage::new(width, height);
    for x in 0..width {
        for y in 0..height {
            let color = pixels[x as usize][y as usize];
            let r = (color[0] * 255.0).round() as u8;
            let g = (color[1] * 255.0).round() as u8;
            let b = (color[2] * 255.0).round() as u8;

            img.put_pixel(x, y, Rgb([r, g, b]));
        }
    }
    println!("{} exported.", path);

    img.save(path).expect("Could not save png");
}
