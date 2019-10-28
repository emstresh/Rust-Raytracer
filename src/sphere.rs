use crate::ray::Ray;
use crate::material::Material;
use crate::hitable::{ Hitable, HitRecord };
use crate::bbox::{ Bounded, BBox };
use crate::util;

use cgmath::{
    dot,
    Vector3
};

pub struct Sphere {
    pub center: Vector3<f32>,
    pub radius: f32,
    pub material: Material
}

impl Sphere {
    pub fn new(center: Vector3<f32>, radius: f32, material: Material) -> Self {
        Self {
            center,
            radius,
            material
        }
    }
}

impl Bounded for Sphere {
    fn bounds(&self, _t0: f32, _t1: f32) -> BBox {
        BBox::new(
            self.center - Vector3::new(self.radius, self.radius, self.radius),
            self.center + Vector3::new(self.radius, self.radius, self.radius)
        )
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = dot(r.direction, r.direction);
        let b = dot(oc, r.direction);
        let c = dot(oc, oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let mut temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let hit_point = r.point_at_parameter(temp);
                let normal = (hit_point - self.center) / self.radius;
                let (u, v) = util::get_sphere_uv(normal);
                return Some(HitRecord {
                    t: temp,
                    p: hit_point,
                    normal,
                    material: &self.material,
                    u,
                    v
                });
            }
            temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let hit_point = r.point_at_parameter(temp);
                let normal = (hit_point - self.center) / self.radius;
                let (u, v) = util::get_sphere_uv(normal);
                return Some(HitRecord {
                    t: temp,
                    p: hit_point,
                    normal,
                    material: &self.material,
                    u,
                    v
                });
            }
        }

        None
    }
}