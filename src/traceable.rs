use std::sync::Arc;

use crate::aabb::AABB;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3d, Vec3};

#[derive(Clone, Debug)]
pub struct RayHit {
  /** Point of hit */
  pub p: Point3d,
  /** Normal at the place of hit. Can point into shape */
  pub normal: Vec3,
  /** Ray distance from origin */
  pub t: f32,
  /** Texture coordinate, x-axis */
  pub u: f32,
  /** Texture coordinate, y-axis */
  pub v: f32,
  /** Is front face */
  pub front_face: bool,
  pub material: Arc<dyn Material>,
}

impl RayHit {
  #[allow(dead_code)]
  pub fn has_hit(&self) -> bool {
    !self.t.is_nan()
  }

  pub fn check_is_front_face(r: &Ray, outward_normal: Vec3) -> (bool, Vec3) {
    let is_front_face = r.dir.dot(outward_normal) < 0.0;
    if is_front_face {
      (true, outward_normal)
    } else {
      (false, !outward_normal)
    }
  }
}

pub trait Traceable: Send + Sync {
  fn check_intersection(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<RayHit>;
  fn bounding_box(&self) -> Option<AABB>;
}
