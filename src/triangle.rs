use crate::ray::Ray;
use crate::material::Material;
use crate::hitable::{ Hitable, HitRecord };
use crate::bbox::{ Bounded, BBox };
use crate::util;

use cgmath::{
    dot,
    InnerSpace,
    Vector3
};

const EPSILON: f32 = 0.0000001;

pub struct Triangle {
    pub v0: Vector3<f32>,
    pub v1: Vector3<f32>,
    pub v2: Vector3<f32>,
    pub material: Material
}

impl Triangle {
    pub fn new(v0: Vector3<f32>, v1: Vector3<f32>, v2: Vector3<f32>, material: Material) -> Self {
        Self {
            v0,
            v1,
            v2,
            material
        }
    }
}

impl Bounded for Triangle {
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

impl Hitable for Triangle {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let edge1 = self.v1 - self.v0;
        let edge2 = self.v2 - self.v0;

        // let n = edge1.cross(edge2);

        // // check if ray and triangle are parallel
        // let n_dot_dir = dot(n, r.direction);
        // if n_dot_dir.abs() < EPSILON {
        //     return None;
        // }

        // let d = dot(n, self.v0);

        // // compute t
        // let t = (dot(n, r.origin) + d) / n_dot_dir.abs(); // TODO: why abs?

        // // check that t is in bounds
        // if t < t_min || t > t_max {
        //     return None;
        // }

        // // compute the intersection point
        // let p = r.point_at_parameter(t);

        // // inside-outside test
        // // vector perpendicular to triangle
        // let mut c = Vector3::new(0.0, 0.0, 0.0);

        // // edge 0
        // c = (self.v1 - self.v0).cross(p - self.v0);
        // if dot(n, c) < 0.0 {
        //     return None;
        // }

        // // edge 1
        // c = (self.v2 - self.v1).cross(p - self.v1);
        // let mut u = dot(n, c);
        // if u < 0.0 {
        //     return None;
        // }

        // // edge 2
        // c = (self.v0 - self.v2).cross(p - self.v2);
        // let mut v = dot(n, c);
        // if v < 0.0 {
        //     return None;
        // }

        // u = u / dot(n, n);
        // v = v / dot(n, n);

        // Some(HitRecord {
        //     t: t,
        //     p: p,
        //     normal: n.normalize(),
        //     material: &self.material,
        //     u: u,
        //     v: v
        // })
        
        //
        //
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
                u: u, // dot(n, edge1.cross(p - self.v1)) / dot(n, n),
                v: v // dot(n, edge2.cross(p - self.v2)) / dot(n, n)
            });
        } else {
            // line intersection but not ray intersection
            return None;
        }
    }
}