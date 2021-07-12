use glam::f32::Mat4;
use std::sync::Arc;

use crate::aabb::AABB;
use crate::ray::Ray;
use crate::traceable::{RayHit, Traceable};
use crate::vec3::Point3d;

// The book has manual math and allows only roration around Y axis.
// I'm not gonna pretend that I don't know the solution, so here
// are the matrices.
//
// From Matt Pharr's blog, PBRT also uses matrices. Tho they use it during runtime too?
// I assumed that offline renderer will parse the files and apply
// the transforms before rendering. And then just deal with Point3d.
// Even animation. ATM not sure why not..

#[derive(Clone)]
pub struct Transform {
  transform: Mat4,
  transform_inverse: Mat4,
  object: Arc<dyn Traceable>,
  aabb: Option<AABB>,
}

impl Transform {
  pub fn new(transform: Mat4, object: Arc<dyn Traceable>) -> Self {
    Transform {
      transform,
      transform_inverse: transform.inverse(),
      object: object.clone(),
      aabb: Transform::calc_bounding_box(transform, object),
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
        Some(AABB::ginormous()) // TODO debug

        // let points = bb.to_points();
        // let tfx_points = &points.iter().map(|p| p.transform_mat4(transform));
        // let a: Vec<Point3d> = tfx_points.to_owned().collect();
        // Some(AABB::from_point_cloud(&a))
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
    let offseted_ray = Ray {
      origin: r.origin.transform_mat4(mat),
      dir: r.dir.transform_mat4(mat), // do not normalize!
    };

    let result = self.object.check_intersection(&offseted_ray, t_min, t_max);
    match result {
      None => None,
      Some(mut hit) => {
        // revert from object to world space
        hit.p = hit.p.transform_mat4(self.transform_inverse);

        // this is.. complicated. I've followed the book, but:
        // 1. I think this may deform normals (inverse for normals was not just a usuall inverse)
        // 2. Recalc if front face - I don't think it is needed.
        hit.normal = hit
          .normal
          .transform_mat4(self.transform_inverse)
          .unit_vector();
        let (is_front_face, outward_normal) =
          RayHit::check_is_front_face(&offseted_ray, hit.normal);
        hit.front_face = is_front_face;
        // hit.normal = outward_normal;
        // rec.set_face_normal(rotated_r, normal); // ?

        Some(hit)
      }
    }
  }
}
