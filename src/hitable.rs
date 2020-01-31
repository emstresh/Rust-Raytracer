use crate::ray::Ray;
use crate::material::Material;
use crate::sphere::Sphere;
use crate::mesh::Mesh;
use crate::moving_sphere::MovingSphere;
use crate::bbox::{ Bounded, BBox };

use cgmath::Vector3;

pub trait Hitable {
  fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub enum Geometry<'material> {
    Sphere(Sphere<'material>),
    MovingSphere(MovingSphere<'material>),
    Mesh(Mesh<'material>)
}

impl<'material> Geometry<'material> {
    pub fn sphere(center: Vector3<f32>, radius: f32, material: &'material Material) -> Geometry<'material> {
        Geometry::Sphere(Sphere::new(center, radius, material))
    }

    pub fn moving_sphere(center0: Vector3<f32>, center1: Vector3<f32>, time0: f32, time1: f32, radius: f32, material: &'material Material) -> Geometry<'material> {
        Geometry::MovingSphere(MovingSphere::new(center0, center1, time0, time1, radius, material))
    }

    pub fn mesh(vertices: Vec<f32>, indices: Vec<usize>, material: &'material Material) -> Geometry<'material> {
        Geometry::Mesh(Mesh::new(vertices, indices, material))
    }
}

impl Bounded for Geometry<'_> {
    fn bounds(&self, t0: f32, t1: f32) -> BBox {
        match self {
            Geometry::Sphere(s) => s.bounds(t0, t1),
            Geometry::MovingSphere(ms) => ms.bounds(t0, t1),
            Geometry::Mesh(m) => m.bounds(t0, t1)
        }
    }
}

impl Hitable for Geometry<'_> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        match self {
            Geometry::Sphere(s) => s.hit(r, t_min, t_max),
            Geometry::MovingSphere(ms) => ms.hit(r, t_min, t_max),
            Geometry::Mesh(m) => m.hit(r, t_min, t_max)
        }
    }
}

pub struct HitRecord<'a> {
    pub t: f32,
    pub p: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub material: &'a Material,
    pub u: f32,
    pub v: f32
}

pub fn bounding_box_list<'a>(items: &'a [Geometry], t0: f32, t1: f32) -> BBox {
    let min = items.iter().fold(
        Vector3::new(std::f32::MAX, std::f32::MAX, std::f32::MAX),
        |acc, item| {
            let bbox = item.bounds(t0, t1);
            Vector3::new(
                acc[0].min(bbox.min[0]),
                acc[1].min(bbox.min[1]),
                acc[2].min(bbox.min[2])
            )
        }
    );

    let max = items.iter().fold(
        Vector3::new(std::f32::MIN, std::f32::MIN, std::f32::MIN),
        |acc, item| {
            let bbox = item.bounds(t0, t1);
            Vector3::new(
                acc[0].max(bbox.max[0]),
                acc[1].max(bbox.max[1]),
                acc[2].max(bbox.max[2])
            )
        }
    );

    return BBox::new(
        min,
        max
    );
}

pub fn hit_list<'a>(items: &'a [Geometry], r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'a>> {
    let mut hit_anything: Option<HitRecord> = None;
    let mut closest_so_far = t_max;
    items.iter().for_each(|item| {
        if let Some(hit) = item.hit(r, t_min, closest_so_far) {
            closest_so_far = hit.t;
            hit_anything = Some(hit);
        }
    });

    hit_anything
}
