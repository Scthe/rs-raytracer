use crate::ray::Ray;
use crate::traceable::{RayHit, Traceable};
use crate::vec3::{Point3d, Vec3};

#[derive(Debug)]
pub struct Camera {
  position: Point3d,
  horizontal: Vec3,
  vertical: Vec3,
  // 3d space coordinates of lower left corner
  lower_left_corner: Point3d,
}

impl Camera {
  pub fn new(
    position: Point3d,
    look_at: Point3d,
    up: Vec3,
    vfov_dgr: f32,     // vertical field of view in degrees e.g. 90
    aspect_ratio: f32, // e.g. 16.0 / 9.0
  ) -> Camera {
    let h = (vfov_dgr.to_radians() / 2.0).tan();
    let viewport_height = 2.0 * h;
    let viewport_width = aspect_ratio * viewport_height;

    let w = (position - look_at).unit_vector();
    let u = up.cross(w).unit_vector();
    let v = w.cross(u).unit_vector(); // actually needed btw.

    let horizontal = u * viewport_width;
    let vertical = v * viewport_height;
    Camera {
      position,
      horizontal,
      vertical,
      lower_left_corner: position - horizontal / 2.0 - vertical / 2.0 - w,
    }
  }

  pub fn get_ray(&self, s: f32, t: f32) -> Ray {
    Ray::new(
      self.position,
      self.lower_left_corner + self.horizontal * s + self.vertical * t - self.position,
    )
  }
}
