use std::sync::Arc;

use crate::aabb::AABB;
use crate::material::Material;
use crate::ray::Ray;
use crate::traceable::{RayHit, Traceable};
use crate::vec3::{Point3d, Vec3};

/** ATM it's just a plane in xy space */
#[derive(Clone, Debug)]
pub struct Rectangle {
  pub x0: f32,
  pub x1: f32,
  pub y0: f32,
  pub y1: f32,
  /** point when following along normal, just like `t` in Ray */
  pub k: f32,
  pub material: Arc<dyn Material>,
}

impl Rectangle {
  pub fn new(p0: (f32, f32), p1: (f32, f32), k: f32, material: Arc<dyn Material>) -> Self {
    Self {
      x0: p0.0.min(p1.0),
      y0: p0.1.min(p1.1),
      x1: p0.0.max(p1.0),
      y1: p0.1.max(p1.1),
      k,
      material,
    }
  }
}

impl Traceable for Rectangle {
  fn bounding_box(&self) -> Option<AABB> {
    // since plane has infini-small depth, we pad it a bit
    Some(AABB {
      min: Point3d::new(self.x0, self.y0, self.k + 0.1),
      max: Point3d::new(self.x1, self.y1, self.k - 0.1),
    })
  }

  fn check_intersection(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<RayHit> {
    // intersection `t` between plane and ray
    let t = (self.k - r.origin.z()) / r.dir.z();
    if t < t_min || t > t_max {
      return None;
    }

    // actual 3d point of ray-plane intersection
    let p = r.at(t);
    if p.x() < self.x0 || p.x() > self.x1 || p.y() < self.y0 || p.y() > self.y1 {
      return None;
    }

    let normal = !Vec3::forward();
    let (front_face, outward_normal) = RayHit::check_is_front_face(r, normal);
    Some(RayHit {
      p,
      t,
      u: (p.x() - self.x0) / (self.x1 - self.x0),
      v: (p.y() - self.y0) / (self.y1 - self.y0),
      normal: outward_normal,
      front_face,
      material: self.material.clone(),
    })
  }
}
