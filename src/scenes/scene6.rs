use glam::f32::Mat4;
use log::info;
use std::path::Path;
use std::sync::Arc;

use super::scene_settings::SceneSettings;
use crate::material::Lambert;
use crate::rectangle::Rectangle;
use crate::sphere::Sphere;
use crate::texture::ImageTex;
use crate::transform::Transform;
use crate::vec3::{Color, Point3d};
use crate::world::World;

#[allow(dead_code)]
pub fn settings() -> SceneSettings {
  SceneSettings {
    camera_position: Point3d::new(5.0, 5.0, 5.0),
    camera_target: Point3d::new(0.0, 0.0, 0.0),
    ..Default::default()
  }
}

#[allow(dead_code)]
pub fn load_scene(world: &mut World) {
  info!("Scene6 is transformation test.");

  let size = 2.0;
  let dgr: f32 = 45.0;

  // debug spheres at corners
  world.add(Arc::new(Sphere::new(
    Point3d::new(-size, 0.0, -size),
    0.1,
    Arc::new(Lambert::from_color(Color::uni(0.0))),
  )));
  world.add(Arc::new(Sphere::new(
    Point3d::new(size, 0.0, -size),
    0.1,
    Arc::new(Lambert::from_color(Color::new(1.0, 0.0, 0.0))),
  )));
  world.add(Arc::new(Sphere::new(
    Point3d::new(-size, 0.0, size),
    0.1,
    Arc::new(Lambert::from_color(Color::new(0.0, 1.0, 0.0))),
  )));
  world.add(Arc::new(Sphere::new(
    Point3d::new(size, 0.0, size),
    0.1,
    Arc::new(Lambert::from_color(Color::new(1.0, 1.0, 0.0))),
  )));

  // rotated plane
  let tex = ImageTex::new(Path::new("assets/test_texture.png"));
  let mat = Arc::new(Lambert::texture(Arc::new(tex)));
  let raw_obj = Rectangle::new((-size, -size), (size, size), 0.0, mat);

  let mat_r = Mat4::from_rotation_x(dgr.to_radians());
  let obj = Transform::new(mat_r, Arc::new(raw_obj));
  world.add(Arc::new(obj));
}
