use std::sync::Arc;
use std::vec::Vec;

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
  fn hit(&self, r: &Ray, t_min: f32, t_max: f32, hit: &mut RayHit) -> bool {
    let mut hit_anything = false;
    let mut closest_so_far = t_max;

    for object in &self.objects {
      let mut temp_rec = RayHit::new();
      if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
        hit_anything = true;
        closest_so_far = temp_rec.t;
        *hit = temp_rec;
      }
    }

    hit_anything
  }
}
