use crate::ray::Ray;
use crate::material::Material;
use crate::hitable::{ Hitable, HitRecord };
use crate::bbox::{ Bounded, BBox };

use cgmath::{
    dot,
    InnerSpace,
    Vector3
};

const EPSILON: f32 = 0.0000001;

pub struct Triangle<'material> {
    pub v0: Vector3<f32>,
    pub v1: Vector3<f32>,
    pub v2: Vector3<f32>,
    pub material: &'material Material
}

impl<'material> Triangle<'material> {
    pub fn new(v0: Vector3<f32>, v1: Vector3<f32>, v2: Vector3<f32>, material: &'material Material) -> Self {
        Self {
            v0,
            v1,
            v2,
            material
        }
    }
}

impl Bounded for Triangle<'_> {
    fn bounds(&self, _t0: f32, _t1: f32) -> BBox {
        BBox::new(
            Vector3::new(
                self.v0.x.min(self.v1.x.min(self.v2.x)) - 0.1,
                self.v0.y.min(self.v1.y.min(self.v2.y)) - 0.1,
                self.v0.z.min(self.v1.z.min(self.v2.z)) - 0.1
            ),
            Vector3::new(
                self.v0.x.max(self.v1.x.max(self.v2.x)) + 0.1,
                self.v0.y.max(self.v1.y.max(self.v2.y)) + 0.1,
                self.v0.z.max(self.v1.z.max(self.v2.z)) + 0.1
            )
        )
    }
}

impl Hitable for Triangle<'_> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let edge1 = self.v1 - self.v0;
        let edge2 = self.v2 - self.v0;

        //
        let h = r.direction.cross(edge2);
        let a = dot(edge1, h);

        //  check if ray is parallel to triangle
        if a > -EPSILON && a < EPSILON {
            return None;
        }

        let f = 1.0 / a;
        let s = r.origin - self.v0;
        let u = f * dot(s, h);

        if u < 0.0 || u > 1.0 {
            return None;
        }

        let q = s.cross(edge1);
        let v = f * dot(r.direction, q);

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        // compute intersection point
        let t = f * dot(edge2, q);
        if t > t_min && t < t_max {
            let n = edge1.cross(edge2);
            let p = r.point_at_parameter(t);
            return Some(HitRecord {
                t: t,
                p: p,
                normal: n.normalize(),
                material: &self.material,
                u: u,
                v: v
            });
        } else {
            // line intersection but not ray intersection
            return None;
        }
    }
}