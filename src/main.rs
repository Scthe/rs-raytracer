use log::{error, info, warn};
use std::rc::Rc;

// TODO Stratified Sampling
// TODO opensubdiv
// TODO all the cool Hyperion tech
// TODO Sintel
// TODO MC

// mod camera;
mod ray;
mod sphere;
mod traceable;
mod utils;
mod vec3;
mod world;

use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::traceable::{RayHit, Traceable};
use crate::utils::{color_f32_to_u8, lerp_vec3, to_0_1};
use crate::vec3::{Color, Point3d, Vec3};
use crate::world::World;

fn trace_ray(r: &Ray, world: &World) -> Color {
  let mut hit = RayHit::new();
  // TODO not infinity, causes z-fighting
  if world.hit(r, 0.0, f32::INFINITY, &mut hit) {
    // return Color::new(1.0, 0.0, 0.0);
    return (hit.normal + Color::new(1.0, 1.0, 1.0)) * 0.5;
  }

  let unit_direction = r.dir.unit_vector();
  let t = to_0_1(unit_direction.y());
  lerp_vec3(Color::new(1.0, 1.0, 1.0), Color::new(0.5, 0.7, 1.0), t)
}

fn main() {
  println!("Start!");

  simple_logger::init().unwrap(); // .filter_level(log::LevelFilter::Debug).init();

  // pretty_env_logger::init();
  log::set_max_level(log::LevelFilter::Trace);
  // log::set_max_level(log::LevelFilter::Error);
  // info!("log::infor");
  // warn!("log::warn");
  // error!("log::error");

  ///////////////////////
  let mut world = World::new();
  let s1 = Sphere::new(Point3d::new(0.0, 0.0, -1.0), 0.5);
  let s2 = Sphere::new(Point3d::new(0.0, -100.5, -1.0), 100.0); // ground;
  world.add(Rc::new(s1));
  world.add(Rc::new(s2));

  ///////////////////////
  // Image
  let aspect_ratio = 16.0 / 9.0;
  let image_width: usize = 400;
  let image_height: usize = (image_width as f32 / aspect_ratio) as usize;

  // Camera
  let viewport_height = 2.0;
  let viewport_width = aspect_ratio * viewport_height;
  let focal_length = 1.0;

  let origin = Point3d::new(0.0, 0.0, 0.0); // camera pos
  let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
  let vertical = Vec3::new(0.0, viewport_height, 0.0);
  // 3d coordinates of lower left corner
  let lower_left_corner =
    origin - (horizontal / 2.0) - (vertical / 2.0) - Vec3::new(0.0, 0.0, focal_length);

  // Render
  let mut img = image::RgbImage::new(image_width as u32, image_height as u32);

  for x in 0..image_width {
    for y in 0..image_height {
      let u = x as f32 / (image_width as f32 - 1.0);
      let v = y as f32 / (image_height as f32 - 1.0);
      let r = Ray::new(
        origin,
        lower_left_corner + horizontal * u + vertical * v - origin,
      );
      let pixel_color = trace_ray(&r, &world);
      img.put_pixel(
        x as u32,
        (image_height - y - 1) as u32,
        image::Rgb(color_f32_to_u8(pixel_color)),
      );
    }
  }
  img.save("output.png").unwrap();

  info!("-- DONE --");
}
