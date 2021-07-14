use glam::f32::Vec3 as gVec3;
use log::info;
use std::sync::Arc;

use crate::box_prim::BoxPrim;
use crate::material::Lambert;
use crate::rectangle::Rectangle;
use crate::scenes::add_debug_spheres;
use crate::traceable::Traceable;
use crate::transform::Transform;
use crate::vec3::{Color, Point3d};
use crate::world::World;

use super::scene_settings::SceneSettings;

#[allow(dead_code)]
pub fn settings() -> SceneSettings {
  SceneSettings {
    // camera_position: Point3d::new(0.0, 0.0, 3.0),
    // camera_position: Point3d::new(3.0, 2.0, 3.0), // 45dgr
    camera_position: Point3d::new(0.0, 4.5, 0.5), // top
    camera_target: Point3d::new(0.0, 0.0, 0.0),
    samples_per_pixel: 5,
    max_bounces: 10,
    ..Default::default()
  }
}
#[allow(dead_code)]
pub fn load_scene(world: &mut World) {
  info!("Scene8 is transforms playground.");

  let size = 1.0;
  let rad = |r: f32| r.to_radians();

  // floor - reference point
  let mat_grey = Arc::new(Lambert::from_color(Color::uni(0.2)));
  let obj = Rectangle::new((-size, -size), (size, size), 0.0, mat_grey);
  let obj = Transform::from_transform_rot(
    glam::f32::Mat3::from_rotation_x(rad(90.0)),
    gVec3::new(0.0, 0.0, 0.0),
    Arc::new(obj),
  );
  world.add(Arc::new(obj));

  // green box
  let mat_box = Arc::new(Lambert::from_color(Color::new(0.0, 0.5, 0.0)));
  let dims = Point3d::new(1.0, 1.0, 1.0);
  let obj = BoxPrim::new(dims, mat_box.clone());

  let bb = obj.bounding_box();
  println!("-- PRE dims {:?}", obj.dims());
  println!("-- PRE AABB {:?}", bb);
  println!("-- PRE AABB.dims {:?}", bb.map(|bb| bb.dims()));

  let obj = Transform::from_transform_rot(
    glam::f32::Mat3::from_rotation_y(rad(45.0)),
    gVec3::new(-1.0, 0.0, 0.0),
    Arc::new(obj),
  );
  let bb = obj.bounding_box();
  world.add(Arc::new(obj));

  println!("-- POST AABB {:?}", bb);
  println!("-- POST AABB.dims {:?}", bb.map(|bb__| bb__.dims()));

  add_debug_spheres(bb, world);
}
