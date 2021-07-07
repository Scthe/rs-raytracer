use crate::ray::Ray;
use crate::vec3::{Point3d, Vec3};

#[derive(Clone, Debug, Copy)]
pub struct RayHit {
  pub p: Point3d,
  pub normal: Vec3,
  pub t: f32,
  pub front_face: bool,
}

impl RayHit {
  pub fn new() -> RayHit {
    RayHit {
      p: Vec3::zero(),
      normal: Vec3::up(),
      t: f32::NAN,
      front_face: false,
    }
  }

  pub fn has_hit(&self) -> bool {
    !self.t.is_nan()
  }

  pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
    self.front_face = r.dir.dot(outward_normal) < 0.0;
    self.normal = if self.front_face {
      outward_normal
    } else {
      !outward_normal
    };
  }
}

pub trait Traceable {
  fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit: &mut RayHit) -> bool;
}
