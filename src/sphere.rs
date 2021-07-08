use crate::material::Material;
use crate::ray::Ray;
use crate::traceable::{RayHit, Traceable};
use crate::vec3::Point3d;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Sphere {
  pub center: Point3d,
  pub radius: f32,
  pub material: Arc<dyn Material>,
}

impl Sphere {
  pub fn new(center: Point3d, radius: f32, material: Arc<dyn Material>) -> Sphere {
    Sphere {
      center,
      radius,
      material,
    }
  }
}

impl Traceable for Sphere {
  fn hit(&self, r: &Ray, t_min: f32, t_max: f32, hit: &mut RayHit) -> bool {
    // solving quadratic equation wrt. `t`:
    //  $ t² * (b•b)  +  2tb•(A-C)  +  (A-C)•(A-C) - r² = 0;
    // where
    //  $ ray = A + tb
    //  $ sphere = {origin=C, radius=r}
    // Usuall way of solving quadratic equation from high school:
    //  $ delta = b² - 4ac // also known as discriminant
    //  $ x = (-b +- sqrt(delta)) / 2a
    // where
    //  $ a = b•b
    //  $ b = 2b•(A-C) // sometimes this is halved and equations differ, but results is same
    //  $ c = (A-C)•(A-C) - r²
    let oc = r.origin - self.center; // origin-center, A-C
    let a = r.dir.length_squared();
    let b = 2.0 * r.dir.dot(oc);
    let c = oc.length_squared() - self.radius * self.radius;

    let delta = b * b - 4.0 * a * c;
    if delta < 0.0 {
      // no solution == no intersection
      return false;
    }
    let delta_sqrt = delta.sqrt();

    // Find the nearest root that lies in the acceptable range.
    let is_in_range = |v| -> bool { v >= t_min && v <= t_max };
    let mut root = (-b - delta_sqrt) / (2.0 * a);
    if !is_in_range(root) {
      root = (-b + delta_sqrt) / (2.0 * a); // solution 2
      if !is_in_range(root) {
        return false;
      }
    }

    hit.t = root;
    hit.p = r.at(hit.t);
    hit.material = self.material.clone();
    let normal = (hit.p - self.center).unit_vector();
    hit.set_face_normal(r, normal);
    true
  }
}
