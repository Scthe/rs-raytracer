use log::info;
use std::sync::Arc;

use crate::light::DiffuseLight;
use crate::material::{Dielectric, Lambert, Metal, SolidColor};
use crate::rectangle::Rectangle;
use crate::sphere::Sphere;
use crate::vec3::{Color, Point3d, Vec3};
use crate::world::World;

#[allow(dead_code)]
pub fn camera() -> (Point3d, Point3d) {
  (Point3d::new(5.0, 2.0, 0.0), Point3d::new(0.0, 0.1, 0.0))
}

#[allow(dead_code)]
pub fn load_scene(world: &mut World) {
  info!("Scene5 is lights test.");

  // ground
  let gray = 0.05;
  let mat_ground = Arc::new(Lambert::color(gray, gray, gray));
  let s_ground = Sphere::new(Point3d::new(0.0, -1000.45, -1.2), 1000.0, mat_ground);
  world.add(Arc::new(s_ground));

  // balls
  let gray = 0.5;
  let mat_grey = Arc::new(Lambert::color(gray, gray, gray));
  let sphere = Sphere::new(Vec3::new(-3.5, 0.45, 1.0), 0.9, mat_grey);
  world.add(Arc::new(sphere));

  let mat_glass_teal = Arc::new(Dielectric {
    albedo: Vec3::new(0.5, 0.7, 0.7),
    ior: 1.3,
  });
  let sphere = Sphere::new(Vec3::new(1.5, 0.45, 0.5), 0.5, mat_glass_teal);
  world.add(Arc::new(sphere));

  // light
  let mat_light = Arc::new(DiffuseLight::color(Color::new(0.5, 0.0, 0.0), 3.0));
  let size = 6.0;
  let light = Rectangle::new((-size, -size), (size, size), 3.0, mat_light);
  world.add(Arc::new(light));
}
