use crate::ray::Ray;
use crate::material::Material;
use crate::hitable::{ Hitable, HitRecord };
use crate::bbox::{ Bounded, BBox };
use crate::util;

use cgmath::{
    dot,
    Vector3
};

pub struct Rectangle {
    pub x0: f32,
    pub x1: f32,
    pub y0: f32,
    pub y1: f32,
    pub k: f32,
    pub material: Material
}

impl Rectangle {
    pub fn new(x0: f32, x1: f32, y0: f32, y1: f32, k: f32, material: Material) -> Self {
        Self {
            x0,
            x1,
            y0,
            y1,
            k,
            material
        }
    }
}

impl Bounded for Rectangle {
    fn bounds(&self, _t0: f32, _t1: f32) -> BBox {
        BBox::new(
            Vector3::new(self.x0, self.y0, self.k - 0.0001),
            Vector3::new(self.x1, self.y1, self.k + 0.0001)
        )
    }
}

impl Hitable for Rectangle {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - r.origin.z) / r.direction.z;
        if t < t_min || t > t_max {
            return None;
        }
        let x = r.origin.x + t * r.direction.x;
        let y = r.origin.y + t * r.direction.y;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }
        Some(HitRecord {
            t: t,
            p: r.point_at_parameter(t),
            normal: Vector3::new(0.0, 0.0, 1.0),
            material: &self.material,
            u: (x - self.x0) / (self.x1 - self.x0),
            v: (y - self.y0) / (self.y1 - self.y0)
        })
    }
}