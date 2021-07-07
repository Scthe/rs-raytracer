use std::fmt;

use crate::vec3::{Point3d, Vec3};

#[derive(Clone, Debug, Copy)]
pub struct Ray {
  pub origin: Point3d,
  pub dir: Vec3,
}

impl Ray {
  pub fn new(origin: Point3d, dir: Vec3) -> Ray {
    Ray { origin, dir }
  }

  pub fn at(self, t: f32) -> Point3d {
    self.origin + (self.dir * t)
  }
}

impl Default for Ray {
  fn default() -> Self {
    Ray::new(Vec3::default(), Vec3::new(0.0, 0.0, -1.0))
  }
}

impl fmt::Display for Ray {
  // This trait requires `fmt` with this exact signature.
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Ray(origin={}, dir={})", self.origin, self.dir)
  }
}
