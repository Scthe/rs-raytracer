use glam::f32::Mat4;
use std::sync::Arc;

use crate::aabb::AABB;
use crate::ray::Ray;
use crate::traceable::{RayHit, Traceable};
use crate::vec3::Point3d;

// The book shows the math for rotation around Y axis with sines and cosines.
// I'm not gonna pretend that I don't know the solution, so here
// are the matrices.
//
// From Matt Pharr's blog, PBRT also uses matrices. Tho they use it during runtime too?
// I assumed that offline renderer will parse the files and apply
// the transforms before rendering. And then just deal with Point3d.
// Even animation. ATM not sure why not..

#[derive(Clone)]
/** 3d transformation, like rotate and move. */
pub struct Transform {
  transform: Mat4,
  transform_inverse: Mat4,
  object: Arc<dyn Traceable>,
  aabb: Option<AABB>,
}

impl Transform {
  /*
  // TODO finish this.
  pub fn new(transform: Mat4, object: Arc<dyn Traceable>) -> Self {
    let aabb = Transform::calc_bounding_box(transform.inverse(), object.clone());
    // let aabb = Transform::calc_bounding_box(transform, object.clone());
    Transform {
      object: object.clone(),
      // transform,
      // transform_inverse: transform.inverse(),
      // aabb: Transform::calc_bounding_box(transform.inverse(), object),
      transform: transform.inverse(), // we manipulate ray, not the object. So moving object right 5u is same as movin ray -5u
      transform_inverse: transform,
      aabb,
    }
  }
  */

  pub fn from_transform_rot(
    mat3: glam::f32::Mat3,
    translation: glam::f32::Vec3,
    object: Arc<dyn Traceable>,
  ) -> Self {
    let tfx0 = Mat4::from_mat3(mat3); // rotation matrix
    let tfx1 = Mat4::from_translation(translation); // translation matrix
    let transform = tfx0 * tfx1;

    // AABB: WTF? Done by trial and error
    let tfx1 = Mat4::from_translation(-translation);
    let transform_aabb = tfx1 * tfx0;
    let aabb = Transform::calc_bounding_box(transform_aabb, object.clone());

    Transform {
      object: object.clone(),
      transform, // we manipulate ray, not the object. So moving object right 5u is same as movin ray -5u
      transform_inverse: transform.inverse(),
      aabb,
    }
  }

  fn calc_bounding_box(
    transform: Mat4, //
    object: Arc<dyn Traceable>,
  ) -> Option<AABB> {
    let obj_bb = object.bounding_box();
    match obj_bb {
      None => None,
      Some(bb) => {
        // Some(AABB::ginormous()) // debug

        // Get child AABB, transform by matrix, recalc AABB to be axis-aligned (in case there was a rotation)
        let points = bb.to_points();
        let tfx_points = &points.iter().map(|p| p.transform_mat4(transform));
        let a: Vec<Point3d> = tfx_points.to_owned().collect();
        Some(AABB::from_point_cloud(&a))
      }
    }
  }
}

impl Traceable for Transform {
  fn bounding_box(&self) -> Option<AABB> {
    self.aabb
  }

  fn check_intersection(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<RayHit> {
    let mat = self.transform;
    // Rotation matrix from our transform. Assumes no scale/shear. Our ray direction is unit vector
    // expressing rotation. It should not be translated, only rotated.
    let rot = glam::f32::Mat3::from_mat4(mat);
    // Express ray from world space into object space (by using matrix)
    let offseted_ray = Ray {
      origin: r.origin.transform_mat4(mat),
      dir: r.dir.transform_mat3(rot), // do not normalize! Tho do not matter that much if we only rotate
    };

    let result = self.object.check_intersection(&offseted_ray, t_min, t_max);
    match result {
      None => None,
      Some(mut hit) => {
        // revert hit point from object to world space.
        // TBH we could probably just do `hit.p = r.at(hit.t)`
        hit.p = hit.p.transform_mat4(self.transform_inverse);

        // this is.. complicated. I've followed the book, but:
        // 1. I think this may deform normals (inverse for normals was not just a usuall inverse)
        // 2. Recalc if front face - I don't think it is needed.
        let normal = hit.normal.transform_mat3(rot.inverse()).unit_vector();
        // transform normal from object into world space. Again, only rotation is applied,
        // since normal is unit-length vector
        hit.normal = normal;

        Some(hit)
      }
    }
  }
}
