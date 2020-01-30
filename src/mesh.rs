use std::f32::{ MIN, MAX };
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

pub struct Mesh<'material> {
    pub vertices: Vec<f32>,
    pub indices: Vec<usize>,
    pub material: &'material Material
}

impl<'material> Mesh<'material> {
    pub fn new(vertices: Vec<f32>, indices: Vec<usize>, material: &'material Material) -> Self {
        Self {
            vertices,
            indices,
            material
        }
    }

    fn hit_triangle(&self, r: &Ray, t_min: f32, t_max: f32, v0: Vector3<f32>, v1: Vector3<f32>, v2: Vector3<f32>) -> Option<HitRecord> {
        let edge1 = v1 - v0;
        let edge2 = v2 - v0;

        //
        let h = r.direction.cross(edge2);
        let a = dot(edge1, h);

        //  check if ray is parallel to triangle
        if a > -EPSILON && a < EPSILON {
            return None;
        }

        let f = 1.0 / a;
        let s = r.origin - v0;
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

impl Bounded for Mesh<'_> {
    fn bounds(&self, _t0: f32, _t1: f32) -> BBox {
        // TODO: cleaner way?
        let (mut x_min, mut x_max, mut y_min, mut y_max, mut z_min, mut z_max) = (MAX, MIN, MAX, MIN, MAX, MIN);
        self.vertices.chunks(3)
            .for_each(|v| {
                x_min = x_min.min(v[0]);
                x_max = x_max.max(v[0]);
                y_min = y_min.min(v[1]);
                y_max = y_max.max(v[1]);
                z_min = z_min.min(v[2]);
                z_max = z_max.max(v[2]);
            });
        BBox::new(
            Vector3::new(
                x_min - 0.1,
                y_min - 0.1,
                z_min - 0.1
            ),
            Vector3::new(
                x_max,
                y_max + 0.1,
                z_max + 0.1
            )
        )
    }
}

impl Hitable for Mesh<'_> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit_anything: Option<HitRecord> = None;
        let mut closest_so_far = t_max;
        self.indices.chunks(3).for_each(|tri| {
            let v0 = Vector3::new(self.vertices[3 * tri[0]], self.vertices[3 * tri[0] + 1], self.vertices[3 * tri[0] + 2]);
            let v1 = Vector3::new(self.vertices[3 * tri[1]], self.vertices[3 * tri[1] + 1], self.vertices[3 * tri[1] + 2]);
            let v2 = Vector3::new(self.vertices[3 * tri[2]], self.vertices[3 * tri[2] + 1], self.vertices[3 * tri[2] + 2]);
            if let Some(hit) = self.hit_triangle(r, t_min, closest_so_far, v0, v1, v2) {
                closest_so_far = hit.t;
                hit_anything = Some(hit);
            }
        });

        hit_anything
    }
}