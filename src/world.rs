use std::sync::Arc;
use std::vec::Vec;

use crate::aabb::AABB;
use crate::ray::Ray;
use crate::traceable::{RayHit, Traceable};

pub struct World {
  pub objects: Vec<Arc<dyn Traceable>>,
}

impl World {
  pub fn new() -> World {
    World {
      objects: Vec::new(),
    }
  }

  #[allow(dead_code)]
  pub fn clear(&mut self) {
    self.objects.clear();
  }

  pub fn add(&mut self, obj: Arc<dyn Traceable>) {
    self.objects.push(obj);
  }
}

impl Traceable for World {
  fn bounding_box(&self) -> Option<AABB> {
    let mut result: Option<AABB> = None;

    for object in &self.objects {
      let obj_aabb = object.bounding_box();
      result = match (obj_aabb, result) {
        (None, _) => return None, // cannot have bounding box if some object do not define one
        (Some(a), None) => Some(a), // first object only
        (Some(a), Some(b)) => Some(AABB::merge(&a, &b)),
      };
    }

    result
  }

  fn check_intersection(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<RayHit> {
    let mut result: Option<RayHit> = None;
    let mut closest_so_far = t_max;

    for object in &self.objects {
      match object.check_intersection(r, t_min, closest_so_far) {
        None => (),
        Some(hit_data) => {
          closest_so_far = hit_data.t;
          result = Some(hit_data);
        }
      };
    }

    result
  }
}
