use log::warn;
use std::sync::Arc;

use crate::material::{Lambert, Metal};
use crate::sphere::Sphere;
use crate::vec3::{Color, Point3d, Vec3};
use crate::world::World;

use super::scene_settings::SceneSettings;

#[allow(dead_code)]
pub fn settings() -> SceneSettings {
  SceneSettings {
    camera_position: Point3d::new(0.0, 2.0, 5.0),
    camera_target: Point3d::new(0.0, 0.1, 0.0),
    ..Default::default()
  }
}

#[allow(dead_code)]
pub fn load_scene(world: &mut World) {
  warn!(
    "Scene3 is BVH test. e.g. on low settings it's 1min with BVH, and I gave up after 1h without"
  );

  let mat_ground = Arc::new(Lambert::color(0.15, 0.3, 0.15)); // DO NOT USE SOLID COLOR HERE!

  // ground
  let s_ground = Sphere::new(Point3d::new(0.0, -1000.45, -1.2), 1000.0, mat_ground);
  world.add(Arc::new(s_ground));

  let cnt = 100;
  let radius = 0.3;
  let total_x = (radius * 2.1) * (cnt as f32);
  let p0 = Point3d::new(-total_x / 2.0, -0.12, -5.0);

  for i in 0..cnt {
    for j in 0..cnt {
      let x_vec = Vec3::new((i as f32) / (cnt as f32) * total_x, 0.0, 0.0);
      let y_vec = Vec3::new(0.0, 0.0, (j as f32) * radius * 3.0);
      let p = p0 + x_vec + y_vec;

      let mat_metal_red = Arc::new(Metal {
        albedo: Color::new(
          rand::random::<f32>(),
          rand::random::<f32>(),
          rand::random::<f32>(),
        ),
        roughness: 0.05,
      });
      let s1 = Sphere::new(p, radius, mat_metal_red);
      world.add(Arc::new(s1));
    }
  }
}
