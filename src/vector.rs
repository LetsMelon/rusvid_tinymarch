use nalgebra::vector;

use crate::math::lerp;
use crate::Vector;

/// create a Vector3 with constant values
pub fn vec3(a: f64) -> Vector {
    vector![a, a, a]
}

/// reflect an input vector about another (the sdf surface normal)
pub fn reflect(v: Vector, normal: Vector) -> Vector {
    return v - normal * 2.0 * v.dot(&normal);
}

pub fn mix_vectors(v1: Vector, v2: Vector, t: f64) -> Vector {
    vector![
        lerp(v1.x, v2.x, t),
        lerp(v1.y, v2.y, t),
        lerp(v1.z, v2.z, t)
    ]
}

pub fn vector_max(v1: Vector, v2: Vector) -> Vector {
    vector![v1.x.max(v2.x), v1.y.max(v2.y), v1.z.max(v2.z)]
}

pub fn multiply_vectors(v1: Vector, v2: Vector) -> Vector {
    vector![v1.x * v2.x, v1.y * v2.y, v1.z * v2.z]
}

pub fn divide_vectors(v1: Vector, v2: Vector) -> Vector {
    vector![v1.x * v2.x, v1.y * v2.y, v1.z * v2.z]
}

pub fn powf_vector(v: Vector, p: f64) -> Vector {
    vector![v.x.powf(p), v.y.powf(p), v.z.powf(p)]
}
