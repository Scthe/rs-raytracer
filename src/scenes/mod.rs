use std::sync::Arc;

use crate::aabb::AABB;
use crate::material::SolidColor;
use crate::sphere::Sphere;
use crate::vec3::Color;
use crate::world::World;

pub mod scene1;
pub mod scene2;
pub mod scene3;
pub mod scene4;
pub mod scene5;
pub mod scene6;
pub mod scene7;
pub mod scene8;
pub mod scene_settings;

pub fn add_debug_spheres(aabb: Option<AABB>, world: &mut World) {
  aabb.map(|a| {
    println!("-- Dbg Spheres: {:?}", a);

    for &p in &a.to_points() {
      let mat_metal_red = Arc::new(SolidColor {
        color: Color::new(
          rand::random::<f32>(),
          rand::random::<f32>(),
          rand::random::<f32>(),
        ),
      });
      println!("-- point: {:?}", p);
      let s1 = Sphere::new(p, 0.04, mat_metal_red);
      world.add(Arc::new(s1));
    }
  });
}
