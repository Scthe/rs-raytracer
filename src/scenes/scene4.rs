use log::info;
use noise::Perlin;
use std::path::Path;
use std::sync::Arc;

use crate::material::Lambert;
use crate::sphere::Sphere;
use crate::texture::{CheckerTex, ImageTex, NoiseTex, UVDebugTex};
use crate::vec3::{Color, Point3d};
use crate::world::World;

#[allow(dead_code)]
pub fn camera() -> (Point3d, Point3d) {
  (Point3d::new(0.0, 2.0, 5.0), Point3d::new(0.0, 0.1, 0.0))
}

#[allow(dead_code)]
pub fn load_scene(world: &mut World) {
  info!("Scene4 is textures test");

  // ground
  let ground_tex = CheckerTex {
    color1: Color::uni(0.3),
    color2: Color::uni(0.8),
    scale: 5.0,
  };
  let mat_ground = Arc::new(Lambert::texture(Arc::new(ground_tex)));
  let s_ground = Sphere::new(Point3d::new(0.0, -1000.45, -1.2), 1000.0, mat_ground);
  world.add(Arc::new(s_ground));

  // sphere 1 - texture as image from disk
  let tex = ImageTex::new(Path::new("assets/test_texture.png"));
  let mat_tex = Arc::new(Lambert::texture(Arc::new(tex)));
  let s1 = Sphere::new(Point3d::new(0.0, 0.45, 0.0), 0.9, mat_tex);
  world.add(Arc::new(s1));

  // sphere 2 - uv debug
  let uv_debug_tex = UVDebugTex {};
  let mat_tex = Arc::new(Lambert::texture(Arc::new(uv_debug_tex)));
  let sphere = Sphere::new(Point3d::new(-2.0, 0.45, 0.0), 0.9, mat_tex);
  world.add(Arc::new(sphere));

  // sphere 3 - noise
  let tex = NoiseTex {
    noise: Perlin::new(),
    scale: 10.0,
  };
  let mat_tex = Arc::new(Lambert::texture(Arc::new(tex)));
  let sphere = Sphere::new(Point3d::new(2.0, 0.45, 0.0), 0.9, mat_tex);
  world.add(Arc::new(sphere));
}
