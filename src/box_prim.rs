use glam::f32::Mat3;
use glam::f32::Vec3 as GVec3;
use std::sync::Arc;

use crate::aabb::AABB;
use crate::material::Material;
use crate::ray::Ray;
use crate::rectangle::Rectangle;
use crate::traceable::{RayHit, Traceable};
use crate::transform::Transform;
use crate::vec3::{Point3d, Vec3};
use crate::world::World;

// BTW: box is reserved Rust keyword

#[derive(Clone, Debug)]
pub struct BoxPrim {
  box_min: Point3d,
  box_max: Point3d,
  side_planes: Arc<World>,
}

impl BoxPrim {
  pub fn new(dim: Vec3, mat: Arc<dyn Material>) -> Self {
    let p0 = dim / -2.0;
    let p1 = dim / 2.0;
    let dgr_90 = 90.0_f32.to_radians();

    let half_dim = |dim| (p0[dim].abs() + p1[dim].abs()) / 2.0;
    let make_rect = |dim0: usize, dim1: usize, k: f32| {
      Arc::new(Rectangle::new(
        (p0[dim0], p0[dim1]),
        (p1[dim0], p1[dim1]),
        k,
        mat.clone(),
      ))
    };

    let mut sides = World::new();

    // top, bottom
    let h = half_dim(1);
    let rect = make_rect(0, 2, h);
    sides.add(Arc::new(Transform::from_transform_rot(
      Mat3::from_rotation_x(dgr_90),
      GVec3::ZERO,
      rect.clone(),
    )));
    sides.add(Arc::new(Transform::from_transform_rot(
      Mat3::from_rotation_x(-dgr_90),
      GVec3::ZERO,
      rect.clone(),
    )));

    // left, right
    let h = half_dim(0);
    let rect = make_rect(2, 1, h);
    sides.add(Arc::new(Transform::from_transform_rot(
      Mat3::from_rotation_y(dgr_90),
      GVec3::ZERO,
      rect.clone(),
    )));
    sides.add(Arc::new(Transform::from_transform_rot(
      Mat3::from_rotation_y(dgr_90),
      GVec3::ZERO,
      rect.clone(),
    )));

    // front, back
    let h = half_dim(2);
    sides.add(make_rect(0, 1, h));
    sides.add(make_rect(0, 1, -h));

    Self {
      box_min: p0,
      box_max: p1,
      side_planes: Arc::new(sides),
    }
  }

  #[allow(dead_code)]
  pub fn dims(&self) -> Vec3 {
    self.box_max - self.box_min
  }
}

impl Traceable for BoxPrim {
  fn bounding_box(&self) -> Option<AABB> {
    Some(AABB {
      min: self.box_min,
      max: self.box_max,
    })
  }

  fn check_intersection(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<RayHit> {
    self.side_planes.check_intersection(r, t_min, t_max)
  }
}
