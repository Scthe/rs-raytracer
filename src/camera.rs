use crate::ray::Ray;
use crate::utils::random_in_disk;
use crate::vec3::{Point3d, Vec3};

#[derive(Debug)]
pub struct Camera {
  position: Point3d,
  horizontal: Vec3,
  vertical: Vec3,
  aperture: f32,
  vec_right_global_space: Vec3,
  vec_up_global_space: Vec3,
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
    aperture: f32,
    focus_dist: f32,
  ) -> Camera {
    let h = (vfov_dgr.to_radians() / 2.0).tan();
    let viewport_height = 2.0 * h;
    let viewport_width = aspect_ratio * viewport_height;

    // btw. .unit_vector() after cross() actually needed
    // btw2. we could create matrix now <3
    let w = (position - look_at).unit_vector(); // forward
    let u = up.cross(w).unit_vector(); // right
    let v = w.cross(u).unit_vector(); // up

    let horizontal = u * viewport_width * focus_dist;
    let vertical = v * viewport_height * focus_dist;
    Camera {
      position,
      aperture,
      horizontal,
      vertical,
      lower_left_corner: position - horizontal / 2.0 - vertical / 2.0 - w * focus_dist,
      vec_right_global_space: u,
      vec_up_global_space: v,
    }
  }

  pub fn get_ray(&self, s: f32, t: f32) -> Ray {
    let lens_radius = self.aperture / 2.0;
    let rd = random_in_disk(lens_radius);
    let offset = self.vec_right_global_space * rd.x() + self.vec_up_global_space * rd.y();

    Ray::new(
      self.position + offset,
      self.lower_left_corner + (self.horizontal * s) + (self.vertical * t) - self.position - offset,
    )
  }
}
