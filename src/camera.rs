use crate::ray::Ray;
use crate::util;

use rand::prelude::*;

use cgmath::{
    Vector3,
    InnerSpace
};

const PI: f32 = std::f32::consts::PI;
const TO_RADIANS: f32 = PI / 180.0;

pub struct Camera {
    lower_left_corner: Vector3<f32>,
    horizontal: Vector3<f32>,
    vertical: Vector3<f32>,
    origin: Vector3<f32>,
    lens_radius: f32,
    u: Vector3<f32>,
    v: Vector3<f32>,
    time0: f32,
    time1: f32
}

impl Camera {
    pub fn new(look_from: Vector3<f32>, look_at: Vector3<f32>, vup: Vector3<f32>,
               vfov: f32, aspect: f32, aperture: f32, focus_dist: f32, time0: f32, time1: f32) -> Self {
        let theta = vfov * TO_RADIANS;
        let half_height = (0.5 * theta).tan();
        let half_width = aspect * half_height;

        let w =(Vector3::new(look_from.x, look_from.y, look_from.z) - look_at).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);

        Self {
            lower_left_corner: look_from - half_width * focus_dist * u - half_height * focus_dist * v - focus_dist * w,
            horizontal: 2.0 * half_width * focus_dist * u,
            vertical: 2.0 * half_height * focus_dist * v,
            origin: look_from,
            u,
            v,
            lens_radius: aperture / 2.0,
            time0,
            time1
        }
    }
    
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let rd = self.lens_radius * util::random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        let time = self.time0 + random::<f32>() * (self.time1 - self.time0);
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + (u * self.horizontal) + (v * self.vertical) - self.origin - offset,
            time
        )
    }
}
