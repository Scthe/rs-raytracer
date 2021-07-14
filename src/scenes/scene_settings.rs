use crate::vec3::{Color, Point3d};

#[derive(Debug)]
pub struct SceneSettings {
  pub camera_position: Point3d,
  pub camera_target: Point3d,
  pub camera_aperture: f32,
  pub camera_fov: f32,
  pub background: Color,
  pub samples_per_pixel: usize,
  pub max_bounces: i32,
}

impl Default for SceneSettings {
  fn default() -> Self {
    Self {
      camera_position: Point3d::new(0.0, 0.0, -3.0),
      camera_target: Point3d::new(0.0, 0.0, 0.0),
      camera_aperture: 0.0,
      camera_fov: 40.0,
      background: Color::one(),
      samples_per_pixel: 50,
      max_bounces: 10,
    }
  }
}
