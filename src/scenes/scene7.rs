use glam::f32::Vec3 as gVec3;
use log::info;
use std::sync::Arc;

use crate::box_prim::BoxPrim;
use crate::light::DiffuseLight;
use crate::material::Lambert;
use crate::rectangle::Rectangle;
use crate::transform::Transform;
use crate::vec3::{Color, Point3d};
// use crate::volumetric::Volumetric;
use crate::world::World;

use super::scene_settings::SceneSettings;

#[allow(dead_code)]
pub fn settings() -> SceneSettings {
  SceneSettings {
    camera_position: Point3d::new(0.0, 1.0, 3.5),
    // camera_position: Point3d::new(3.0, 2.0, 3.0) * 2.0, // 45dgr
    // camera_position: Point3d::new(0.0, 4.5, 0.5), // top
    camera_target: Point3d::new(0.0, 1.0, 0.0),
    background: Color::uni(0.0),
    // samples_per_pixel: 150,
    // max_bounces: 20,
    ..Default::default()
  }
}
#[allow(dead_code)]
pub fn load_scene(world: &mut World) {
  info!("Scene7 is Cornell box with volume transmission.");

  let size = 1.0;
  let size_light = 0.3;
  let k = 0.0;
  let rad = |r: f32| r.to_radians();

  let mat_grey = Arc::new(Lambert::from_color(Color::uni(0.2)));
  let mat_red = Arc::new(Lambert::color(1.0, 0.0, 0.0));
  let mat_teal = Arc::new(Lambert::color(0.0, 1.0, 1.0));

  let rect_grey = Arc::new(Rectangle::new((-size, -size), (size, size), k, mat_grey));
  let rect_red = Arc::new(Rectangle::new((-size, -size), (size, size), k, mat_red));
  let rect_teal = Arc::new(Rectangle::new((-size, -size), (size, size), k, mat_teal));

  // light
  let mat_light = Arc::new(DiffuseLight::color(Color::one(), 20.0));
  let light_rect = Rectangle::new(
    (-size_light, -size_light),
    (size_light, size_light),
    0.0,
    mat_light,
  );
  let light_top = Transform::from_transform_rot(
    glam::f32::Mat3::from_rotation_x(rad(90.0)),
    gVec3::new(0.0, -size * 2.0 + 0.01, 0.0),
    Arc::new(light_rect),
  );
  world.add(Arc::new(light_top));

  // floor
  let obj = Transform::from_transform_rot(
    glam::f32::Mat3::from_rotation_x(rad(90.0)),
    gVec3::ZERO,
    rect_grey.clone(),
  );
  world.add(Arc::new(obj));

  // celling
  let obj = Transform::from_transform_rot(
    glam::f32::Mat3::from_rotation_x(rad(90.0)),
    gVec3::new(0.0, -size * 2.0, 0.0),
    rect_grey.clone(),
  );
  world.add(Arc::new(obj));

  // back
  let obj = Transform::from_transform_rot(
    glam::f32::Mat3::IDENTITY,
    gVec3::new(0.0, -1.0, 1.0),
    rect_grey.clone(),
  );
  world.add(Arc::new(obj));

  // left
  let obj = Transform::from_transform_rot(
    glam::f32::Mat3::from_rotation_y(rad(90.0)),
    gVec3::new(1.0, -1.0, 0.0),
    rect_red,
  );
  world.add(Arc::new(obj));

  // right
  let obj = Transform::from_transform_rot(
    glam::f32::Mat3::from_rotation_y(-rad(90.0)),
    gVec3::new(-1.0, -1.0, 0.0),
    rect_teal,
  );
  world.add(Arc::new(obj));

  let mat_box = Arc::new(Lambert::from_color(Color::uni(1.0)));
  // let density: f32 = 2.2;

  // box1 - left
  let dims = Point3d::new(0.35, 0.8, 0.35);
  let obj = BoxPrim::new(dims, mat_box.clone());
  let obj = Transform::from_transform_rot(
    glam::f32::Mat3::from_rotation_y(rad(25.0)),
    gVec3::new(0.5, -0.5, 0.2),
    Arc::new(obj),
  );
  // let obj = Volumetric::color(Arc::new(obj), density, Color::zero());
  world.add(Arc::new(obj));

  // box2 - left
  let dims = Point3d::new(0.4, 0.4, 0.4);
  let obj = BoxPrim::new(dims, mat_box.clone());
  let obj = Transform::from_transform_rot(
    glam::f32::Mat3::from_rotation_y(rad(-25.0)),
    gVec3::new(-0.3, -0.4, -0.3),
    Arc::new(obj),
  );
  // let obj = Volumetric::color(Arc::new(obj), density, Color::one());
  world.add(Arc::new(obj));
}
