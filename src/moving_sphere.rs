use crate::ray::Ray;
use crate::material::Material;
use crate::hitable::{ Hitable, HitRecord };
use crate::bbox::{ Bounded, BBox };
use crate::util;

use cgmath::{
    dot,
    Vector3
};

pub struct MovingSphere {
    pub center0: Vector3<f32>,
    pub center1: Vector3<f32>,
    pub time0: f32,
    pub time1: f32,
    pub radius: f32,
    pub material: Material
}

impl MovingSphere {
    pub fn new(center0: Vector3<f32>, center1: Vector3<f32>, time0: f32, time1: f32, radius: f32, material: Material) -> Self {
        Self {
            center0,
            center1,
            time0,
            time1,
            radius,
            material
        }
    }

    pub fn center(&self, time: f32) -> Vector3<f32> {
        self.center0 + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl Bounded for MovingSphere {
    fn bounds(&self, t0: f32, t1: f32) -> BBox {
        let boxt0 = BBox::new(
            self.center(t0) - Vector3::new(self.radius, self.radius, self.radius),
            self.center(t0) + Vector3::new(self.radius, self.radius, self.radius)
        );
        let boxt1 = BBox::new(
            self.center(t1) - Vector3::new(self.radius, self.radius, self.radius),
            self.center(t1) + Vector3::new(self.radius, self.radius, self.radius)
        );

        boxt0.merge(&boxt1)
    }
}

impl Hitable for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin - self.center(r.time);
        let a = dot(r.direction, r.direction);
        let b = dot(oc, r.direction);
        let c = dot(oc, oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let mut temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let hit_point = r.point_at_parameter(temp);
                let normal = (hit_point - self.center(r.time)) / self.radius;
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
                let normal = (hit_point - self.center(r.time)) / self.radius;
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