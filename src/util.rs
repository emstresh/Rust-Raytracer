use cgmath::{
    Vector3,
    InnerSpace,
    dot
};

use rand::prelude::*;
use std::f32::consts;

const TWO_PI: f32 = 2.0 * consts::PI;

pub fn random_in_unit_sphere() -> Vector3<f32> {
    let mut p = Vector3::new(1.0, 1.0, 1.0);
    let unit = Vector3::new(1.0, 1.0, 1.0);
    while p.magnitude2() >= 1.0 {
        p.x = random::<f32>();
        p.y = random::<f32>();
        p.z = random::<f32>();
        p = 2.0 * p - unit;
    }
    p
}

pub fn random_in_unit_disk() -> Vector3<f32> {
    let mut p = Vector3::new(1.0, 1.0, 0.0);
    let unit = Vector3::new(1.0, 1.0, 0.0);
    while dot(p, p) >= 1.0 {
        p.x = random::<f32>();
        p.y = random::<f32>();
        p = 2.0 * p - unit;
    }
    p
}

pub fn get_sphere_uv(p: Vector3<f32>) -> (f32, f32) {
    let phi = p.z.atan2(p.x);
    let theta = p.y.asin();
    (1.0 - (phi + consts::PI) / TWO_PI, (theta + consts::FRAC_PI_2) / consts::PI)
}
