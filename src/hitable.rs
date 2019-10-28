use crate::ray::Ray;
use crate::material::Material;
use crate::sphere::Sphere;
use crate::rect::Rectangle;
use crate::moving_sphere::MovingSphere;
use crate::bbox::{ Bounded, BBox };

use cgmath::Vector3;

pub trait Hitable {
  fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub enum Geometry {
    Sphere(Sphere),
    MovingSphere(MovingSphere),
    Rectangle(Rectangle)
}

impl Geometry {
    pub fn sphere(center: Vector3<f32>, radius: f32, material: Material) -> Geometry {
        Geometry::Sphere(Sphere::new(center, radius, material))
    }

    pub fn moving_sphere(center0: Vector3<f32>, center1: Vector3<f32>, time0: f32, time1: f32, radius: f32, material: Material) -> Geometry {
        Geometry::MovingSphere(MovingSphere::new(center0, center1, time0, time1, radius, material))
    }

    pub fn rectangle(x0: f32, x1: f32, y0: f32, y1: f32, k: f32, material: Material) -> Geometry {
        Geometry::Rectangle(Rectangle::new(x0, x1, y0, y1, k, material))
    }
}

impl Bounded for Geometry {
    fn bounds(&self, t0: f32, t1: f32) -> BBox {
        match self {
            Geometry::Sphere(s) => s.bounds(t0, t1),
            Geometry::MovingSphere(ms) => ms.bounds(t0, t1),
            Geometry::Rectangle(r) => r.bounds(t0, t1)
        }
    }
}

impl Hitable for Geometry {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        match self {
            Geometry::Sphere(s) => s.hit(r, t_min, t_max),
            Geometry::MovingSphere(ms) => ms.hit(r, t_min, t_max),
            Geometry::Rectangle(re) => re.hit(r, t_min, t_max)
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

// pub struct GeometryList {
//     pub items: Vec<Geometry>
// }

// impl GeometryList {
//     pub fn new(capacity: usize) -> Self {
//         Self {
//             items: Vec::with_capacity(capacity)
//         }
//     }

//     pub fn bounding_box(&self, t0: f32, t1: f32) -> BBox {
//         if self.items.len() > 1 {
//             let min = self.items.iter().fold(
//                 Vector3::new(std::f32::MAX, std::f32::MAX, std::f32::MAX),
//                 |acc, item| {
//                     if let Some(bbox) = item.bounds(t0, t1) {
//                         Vector3::new(
//                             acc[0].min(bbox.min[0]),
//                             acc[1].min(bbox.min[1]),
//                             acc[2].min(bbox.min[2])
//                         )
//                     } else {
//                         acc
//                     }
//                 }
//             );

//             let max = self.items.iter().fold(
//                 Vector3::new(std::f32::MIN, std::f32::MIN, std::f32::MIN),
//                 |acc, item| {
//                     if let Some(bbox) = item.bounds(t0, t1) {
//                         Vector3::new(
//                             acc[0].max(bbox.max[0]),
//                             acc[1].max(bbox.max[1]),
//                             acc[2].max(bbox.max[2])
//                         )
//                     } else {
//                         acc
//                     }
//                 }
//             );

//             return BBox::new(
//                 min,
//                 max
//             );
//         }

//         None
//     }

//     pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
//         let mut hit_anything: Option<HitRecord> = None;
//         let mut closest_so_far = t_max;
//         self.items.iter().for_each(|item| {
//             if let Some(hit) = item.hit(r, t_min, closest_so_far) {
//                 closest_so_far = hit.t;
//                 hit_anything = Some(hit);
//             }
//         });

//         hit_anything
//     }
// }

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
