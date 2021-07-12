use log::info;
use std::sync::Arc;

use crate::material::{Dielectric, Lambert, Metal, SolidColor};
use crate::sphere::Sphere;
use crate::vec3::{Point3d, Vec3};
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
  info!("Scene2 is materials test. Metal, glass, lambert diffuse etc.");

  let mat_ground = Arc::new(Lambert::color(0.15, 0.3, 0.15)); // DO NOT USE SOLID COLOR HERE!

  let mat_grey = Arc::new(SolidColor { color: Vec3::one() });
  let mat_blue = Arc::new(Lambert::color(0.4, 0.45, 0.6));
  let mat_metal_black = Arc::new(Metal {
    albedo: Vec3::uni(0.2),
    roughness: 0.1,
  });
  let mat_metal_silver = Arc::new(Metal {
    albedo: Vec3::uni(0.9),
    roughness: 0.0,
  });
  let mat_metal_red = Arc::new(Metal {
    albedo: Vec3::new(0.7, 0.3, 0.3),
    roughness: 0.5,
  });
  let (ior, glass_dark, glass_light) = (1.3, 0.5, 0.7);
  let mat_glass_red = Arc::new(Dielectric {
    albedo: Vec3::new(glass_light, glass_dark, glass_dark),
    ior,
  });
  let mat_glass_green = Arc::new(Dielectric {
    albedo: Vec3::new(glass_dark, glass_light, glass_dark),
    ior,
  });
  let mat_glass_blue = Arc::new(Dielectric {
    albedo: Vec3::new(glass_dark, glass_dark, glass_light),
    ior,
  });
  let mat_glass_teal = Arc::new(Dielectric {
    albedo: Vec3::new(glass_dark, glass_light, glass_light),
    ior,
  });

  // ground
  let s_ground = Sphere::new(Point3d::new(0.0, -1000.45, -1.2), 1000.0, mat_ground);
  world.add(Arc::new(s_ground));

  // big balls
  let big_radius = 0.9;
  let big_margin_x = big_radius * 2.1;
  let big_total_x = big_margin_x * 4.0;
  let big_left = Point3d::new(-big_total_x / 2.0, 0.45, -3.5);
  let big_margin_between = Vec3::new(big_margin_x, 0.0, 0.0);
  let big_pos = |i: f32| {
    let v_offset = Vec3::new(0.0, 0.0, -1.0) * (2.0 - (2.0 - i).abs());
    big_left + big_margin_between * i + v_offset
  };

  let big1 = Sphere::new(big_pos(0.0), big_radius, mat_grey);
  let big2 = Sphere::new(big_pos(1.0), big_radius, mat_metal_black);
  let big3 = Sphere::new(big_pos(2.0), big_radius, mat_blue);
  let big4 = Sphere::new(big_pos(3.0), big_radius, mat_metal_silver);
  let big5 = Sphere::new(big_pos(4.0), big_radius, mat_metal_red);
  world.add(Arc::new(big1));
  world.add(Arc::new(big2));
  world.add(Arc::new(big3));
  world.add(Arc::new(big4));
  world.add(Arc::new(big5));

  // small balls
  let small_radius = 0.3;
  let small_margin_x = small_radius * 2.1;
  let small_total_x = small_margin_x * 3.0;
  let small_left = Point3d::new(-small_total_x / 2.0, -0.12, -0.5);
  let small_margin_between = Vec3::new(small_margin_x, 0.0, 0.0);
  let small_pos = |i: u32| small_left + small_margin_between * (i as f32);

  let s1 = Sphere::new(small_pos(0), small_radius, mat_glass_red);
  let s2 = Sphere::new(small_pos(1), small_radius, mat_glass_green);
  let s3 = Sphere::new(small_pos(2), small_radius, mat_glass_blue);
  let s4 = Sphere::new(small_pos(3), small_radius, mat_glass_teal);
  world.add(Arc::new(s1));
  world.add(Arc::new(s2));
  world.add(Arc::new(s3));
  world.add(Arc::new(s4));
}
