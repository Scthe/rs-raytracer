use std::sync::Arc;

use crate::material::{Dielectric, Lambert, Metal};
use crate::sphere::Sphere;
use crate::vec3::{Color, Point3d, Vec3};
use crate::world::World;

use super::scene_settings::SceneSettings;

#[allow(dead_code)]
pub fn settings() -> SceneSettings {
  SceneSettings {
    camera_position: Point3d::new(3.0, 3.0, 2.0),
    camera_target: Point3d::new(0.0, 0.0, -1.0),
    camera_aperture: 1.5,
    ..Default::default()
  }
}

#[allow(dead_code)]
pub fn load_scene(world: &mut World) {
  let mat_1 = Arc::new(Lambert::color(0.3, 0.3, 0.7));
  let mat_ground = Arc::new(Lambert::from_color(Color::uni(0.3)));
  let mat_metal = Arc::new(Metal {
    albedo: Vec3::uni(0.8),
    roughness: 0.2,
  });
  let mat_glass = Arc::new(Dielectric {
    ior: 1.5,
    albedo: Color::one(),
  });

  //
  let s1 = Sphere::new(Point3d::new(0.0, 0.0, -1.0), 0.5, mat_1);
  let s_ground = Sphere::new(Point3d::new(0.0, -100.5, -1.0), 100.0, mat_ground); // ground;
  let s_left = Sphere::new(Point3d::new(-1.0, 0.0, -1.0), 0.5, mat_metal);
  let s_right = Sphere::new(Point3d::new(1.0, 0.0, -1.0), 0.5, mat_glass);
  world.add(Arc::new(s1));
  world.add(Arc::new(s_ground));
  world.add(Arc::new(s_left));
  world.add(Arc::new(s_right));
}
