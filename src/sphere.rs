use crate::ray::Ray;
use crate::hitable::{ HitRecord };
use crate::material::Material;

use cgmath::{
    dot,
    Point3
};

pub struct Sphere {
    pub center: Point3<f32>,
    pub radius: f32,
    pub material: Material
}

const EPS: i32 = 1e4;

impl Sphere {
    pub fn new(center: Point3<f32>, radius: f32, material: Material) -> Self {
        Self {
            center,
            radius,
            material
        }
    }

    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = dot(r.direction, r.direction);
        let b = dot(oc, r.direction);
        let c = dot(oc, oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let mut temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let hit_point = r.point_at_parameter(temp);
                return Some(HitRecord {
                    t: temp,
                    p: hit_point,
                    normal: (hit_point - self.center) / self.radius,
                    material: &self.material
                });
            }
            temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let hit_point = r.point_at_parameter(temp);
                return Some(HitRecord {
                    t: temp,
                    p: hit_point,
                    normal: (hit_point - self.center) / self.radius,
                    material: &self.material
                });
            }
        }

        None
    }
}
