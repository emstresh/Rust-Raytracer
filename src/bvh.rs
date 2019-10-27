use crate::bbox::{ Bounded, BBox };
use crate::hitable::{ Geometry, HitRecord, hit_list, bounding_box_list };
use crate::ray::Ray;

use rand::prelude::*;

use std::cmp::Ordering;

// pub struct BVH<T: Bounded> {
//   geometry: Vec<T>
// }

// impl<T: Bounded> BVH<T> {
//   pub fn intersect
// }

pub struct BvhNode<'a> {
  pub bbox: BBox,
  pub item: Option<&'a Geometry>,
  // pub items: &'a [Geometry],
  pub left: Option<Box<BvhNode<'a>>>,
  pub right: Option<Box<BvhNode<'a>>>
}

impl BvhNode<'_> {
  pub fn new(list: &mut [Geometry], t0: f32, t1: f32) -> BvhNode {
    let scene_bbox = bounding_box_list(list, t0, t1);

    let axis = 3 * random::<f32>() as usize;
    list.sort_by(|a, b| box_compare(axis, a, b));

    let len = list.len();

    if len == 1 {
      Self {
        bbox: scene_bbox, //bounding_box_list(&list, t0, t1),
        // items: list,
        item: Some(&list[0]),
        left: None,
        right: None
      }
    } else {
      let (l, r) = list.split_at_mut(len / 2);
      Self {
        bbox: scene_bbox, //bounding_box_list(&list, t0, t1),
        item: None, // list,
        left: Some(Box::new(BvhNode::new(l, t0, t1))),
        right: Some(Box::new(BvhNode::new(r, t0, t1)))
      }
    }
  }

  pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
    if self.bbox.did_hit(r, t_min, t_max) {
      match &self.item {
        Some(item) => item.hit(r, t_min, t_max),
        None => match (&self.left, &self.right) {
          (Some(left), Some(right)) => match (left.hit(r, t_min, t_max), right.hit(r, t_min, t_max)) {
            (Some(hit_left), Some(hit_right)) => if hit_left.t < hit_right.t { Some(hit_left) } else { Some(hit_right) },
            (Some(hit_left), None) => Some(hit_left),
            (None, Some(hit_right)) => Some(hit_right),
            (None, None) => None
          },
          (Some(left), None) => left.hit(r, t_min, t_max),
          (None, Some(right)) => right.hit(r, t_min, t_max),
          (None, None) => None
        }
      };
    };

    None
  }
}

fn box_compare(axis: usize, a: &Geometry, b: &Geometry) -> Ordering {
  let (left_box, right_box) = (a.bounds(0.0, 0.0), b.bounds(0.0, 0.0));
  if left_box.min[axis] - right_box.min[axis] < 0.0 {
    Ordering::Less
  } else {
    Ordering::Greater
  }
}
