use cgmath::Vector3;

use crate::ray::Ray;
// use crate::hitable::HitRecord;

pub trait Bounded {
  fn bounds(&self, t0: f32, t1: f32) -> BBox;
}

pub struct BBox {
  pub min: Vector3<f32>,
  pub max: Vector3<f32>
}

impl BBox {
  pub fn new(min: Vector3<f32>, max: Vector3<f32>) -> Self {
    Self {
      min,
      max
    }
  }

  pub fn merge(&self, other: &BBox) -> BBox {
    BBox {
      min: Vector3::new(
        self.min.x.min(other.min.x),
        self.min.y.min(other.min.y),
        self.min.z.min(other.min.z)
      ),
      max: Vector3::new(
        self.max.x.max(other.max.x),
        self.max.y.max(other.max.y),
        self.max.z.max(other.max.z)
      )
    }
  }

  pub fn did_hit(&self, r: &Ray, t_min: f32, t_max: f32) -> bool {
    for i in 0..3 {
      let inv_d = 1.0 / r.direction[i];
      let mut t0 = (self.min[i] - r.origin[i]) * inv_d;
      let mut t1 = (self.max[i] - r.origin[i]) * inv_d;
      if inv_d < 0.0 {
        std::mem::swap(&mut t0, &mut t1);
      }
      let tmin = if t0 > t_min { t0 } else { t_min };
      let tmax = if t1 < t_max { t1 } else { t_max };

      if tmax <= tmin {
        return false;
      }
    }

    true
  }
}