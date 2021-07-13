use glam::f32::Mat4;
use glam::f32::Vec3 as gVec3;
use log::info;
use std::sync::Arc;

use crate::box_prim::BoxPrim;
use crate::light::DiffuseLight;
use crate::material::Lambert;
use crate::rectangle::Rectangle;
use crate::transform::Transform;
use crate::vec3::{Color, Point3d};
use crate::volumetric::Volumetric;
use crate::world::World;

use super::scene_settings::SceneSettings;

#[allow(dead_code)]
pub fn settings() -> SceneSettings {
  SceneSettings {
    camera_position: Point3d::new(0.0, 0.0, -3.0),
    // camera_position: Point3d::new(3.0, 2.0, -3.0),
    camera_target: Point3d::new(0.0, 0.0, 0.0),
    // background: Color::zero(),
    samples_per_pixel: 50,
    max_bounces: 10,
    ..Default::default()
  }
}

#[allow(dead_code)]
pub fn load_scene(world: &mut World) {
  info!("Scene7 is Cornell box with volume transmission.");

  let size = 1.0; // TODO bigger?
  let size_light = 0.2;
  let dgr_90 = 90.0_f32.to_radians();

  let mat_grey = Arc::new(Lambert::from_color(Color::uni(0.2)));
  let mat_red = Arc::new(Lambert::color(1.0, 0.0, 0.0));
  let mat_teal = Arc::new(Lambert::color(0.0, 1.0, 1.0));

  let rect_grey = Arc::new(Rectangle::new((-size, -size), (size, size), size, mat_grey));
  let rect_red = Arc::new(Rectangle::new((-size, -size), (size, size), size, mat_red));
  let rect_teal = Arc::new(Rectangle::new((-size, -size), (size, size), size, mat_teal));

  // light
  let mat_light = Arc::new(DiffuseLight::color(Color::one(), 5.0));
  let light = Rectangle::new(
    (-size_light, -size_light),
    (size_light, size_light),
    0.99,
    mat_light,
  );
  let tfx = Mat4::from_rotation_x(dgr_90);
  let light_top = Transform::new(tfx, Arc::new(light));
  world.add(Arc::new(light_top));

  // floor
  let tfx = Mat4::from_rotation_x(-dgr_90);
  let floor = Transform::new(tfx, rect_grey.clone());
  world.add(Arc::new(floor));

  // celling
  let tfx = Mat4::from_rotation_x(dgr_90);
  let celling = Transform::new(tfx, rect_grey.clone());
  world.add(Arc::new(celling));

  // back
  world.add(rect_grey);

  // left
  let tfx = Mat4::from_rotation_y(-dgr_90);
  let obj = Transform::new(tfx, rect_red);
  world.add(Arc::new(obj));

  // right
  let tfx = Mat4::from_rotation_y(dgr_90);
  let obj = Transform::new(tfx, rect_teal);
  world.add(Arc::new(obj));

  // TODO add glass sphere infront
  let dgr_30 = 30.0_f32.to_radians();
  let dgr_15 = 15.0_f32.to_radians();
  let mat_box = Arc::new(Lambert::from_color(Color::uni(1.0)));
  let density = 0.7;

  // box1 - left
  let dims = Point3d::new(0.4, 0.8, 0.4);
  let tfx0 = Mat4::from_rotation_y(-dgr_15);
  let tfx1 = Mat4::from_translation(gVec3::new(-0.07, 0.12, -0.07));
  let obj = BoxPrim::new(dims, mat_box.clone());
  let obj = Transform::new(tfx0, Arc::new(obj));
  let obj = Transform::new(tfx1, Arc::new(obj));
  let obj = Volumetric::color(Arc::new(obj), density, Color::zero());
  world.add(Arc::new(obj));

  // box2 - left
  let dims = Point3d::new(0.4, 0.4, 0.4);
  let tfx0 = Mat4::from_rotation_y(dgr_30);
  let tfx1 = Mat4::from_translation(gVec3::new(0.07, 0.2, 0.0));
  let obj = BoxPrim::new(dims, mat_box);
  let obj = Transform::new(tfx0, Arc::new(obj));
  let obj = Transform::new(tfx1, Arc::new(obj));
  let obj = Volumetric::color(Arc::new(obj), density, Color::one());
  world.add(Arc::new(obj));
}
