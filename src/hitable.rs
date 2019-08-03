use crate::ray::Ray;
use crate::material::Material;
use crate::sphere::Sphere;

use cgmath::{
    Vector3,
    Point3
};

pub enum Hitable {
    Sphere(Sphere)
}

impl Hitable {
    pub fn sphere(center: Point3<f32>, radius: f32, material: Material) -> Hitable {
        Hitable::Sphere(Sphere::new(center, radius, material))
    }

    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        match self {
            Hitable::Sphere(s) => s.hit(r, t_min, t_max)
        }
    }
}

pub struct HitRecord<'a> {
    pub t: f32,
    pub p: Point3<f32>,
    pub normal: Vector3<f32>,
    pub material: &'a Material
}

pub struct HitableList {
    pub items: Vec<Hitable>
}

impl HitableList {
    pub fn new(capacity: usize) -> Self {
        Self {
            items: Vec::with_capacity(capacity)
        }
    }

    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit_anything: Option<HitRecord> = None;
        let mut closest_so_far = t_max;
        self.items.iter().for_each(|item| {
            if let Some(hit) = item.hit(r, t_min, closest_so_far) {
                closest_so_far = hit.t;
                hit_anything = Some(hit);
            }
        });

        hit_anything
    }
}
