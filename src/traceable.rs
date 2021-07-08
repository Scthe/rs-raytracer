use crate::material::{Material, SolidColor};
use crate::ray::Ray;
use crate::vec3::{Color, Point3d, Vec3};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct RayHit {
  pub p: Point3d,
  pub normal: Vec3,
  pub t: f32,
  pub front_face: bool,
  pub material: Arc<dyn Material>,
}

static DEFAULT_MATERIAL: SolidColor = SolidColor {
  color: Color::new(1.0, 1.0, 0.0),
};

impl RayHit {
  pub fn new() -> RayHit {
    RayHit {
      p: Vec3::zero(),
      normal: Vec3::up(),
      t: f32::NAN,
      front_face: false,
      material: Arc::new(DEFAULT_MATERIAL.clone()), // TODO ugh, a new copy?
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

pub trait Traceable: Send + Sync {
  fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit: &mut RayHit) -> bool;
}
