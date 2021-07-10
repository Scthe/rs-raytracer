use log::info;
use rand::Rng;
use rayon::prelude::*;

// TODO Stratified Sampling
// TODO opensubdiv
// TODO all the cool Hyperion tech
// TODO Sintel
// TODO Monte Carlo/Metropolis etc.
// TODO SIMD
// TODO CUDA

mod aabb;
mod bvh;
mod camera;
mod light;
mod material;
mod ray;
mod rectangle;
mod scenes;
mod sphere;
mod texture;
mod traceable;
mod utils;
mod vec3;
mod world;

use crate::bvh::BVHNode;
use crate::camera::Camera;
use crate::ray::Ray;
use crate::traceable::Traceable;
use crate::utils::{color_f32_to_u8, gamma_correct, lerp_vec3, to_0_1};
use crate::vec3::{Color, Vec3};
use crate::world::World;

const ACNE_CORRECTION: f32 = 0.001;

fn trace_ray(r: &Ray, world: &dyn Traceable, depth: i32) -> Color {
  if depth <= 0 {
    return Color::zero();
  }

  let result = world.check_intersection(r, ACNE_CORRECTION, f32::INFINITY);
  match result {
    Some(hit) => {
      let bsdf_result = hit.material.bsdf(r, &hit);
      match bsdf_result.bounce {
        Some(r) => {
          return bsdf_result.emissive + bsdf_result.diffuse * trace_ray(&r, world, depth - 1);
        }
        _ => {
          return bsdf_result.emissive;
        }
      }
    }
    _ => {
      // return background
      let unit_direction = r.dir.unit_vector();
      let t = to_0_1(unit_direction.y());
      return lerp_vec3(Color::new(1.0, 1.0, 1.0), Color::new(0.5, 0.7, 1.0), t);
    }
  };
}

fn main() {
  // simple_logger::init().unwrap(); // .filter_level(log::LevelFilter::Debug).init();
  simple_logger::SimpleLogger::new().init().unwrap();
  log::set_max_level(log::LevelFilter::Trace);

  info!("-- START! --");

  ///////////////////////
  // World
  let mut world = World::new();
  scenes::scene5::load_scene(&mut world);
  let (cam_position, cam_look_at) = scenes::scene5::camera();

  ///////////////////////
  // BVH
  info!("-- Building BVH --");
  let bvh = BVHNode::build(&world);

  ///////////////////////
  // Camera
  let aspect_ratio = 16.0 / 9.0;
  let dist_to_focus = (cam_position - cam_look_at).length();
  let aperture = 0.0;
  let cam_fov = 40.0; // scene1: 90.0
  let camera = Camera::new(
    cam_position,
    cam_look_at,
    Vec3::up(),
    cam_fov,
    aspect_ratio,
    aperture,
    dist_to_focus,
  );

  ///////////////////////
  // Render
  info!("-- Tracing rays --");
  let samples_per_pixel: usize = 50; // spp
  let sample_max_bounces: i32 = 10;
  let image_width: u32 = 400;
  let image_height: u32 = (image_width as f32 / aspect_ratio) as u32;
  let mut img = image::RgbImage::new(image_width as u32, image_height as u32);

  let data: Vec<(u32, u32, Color)> = (0..(image_width * image_height))
    .into_par_iter()
    .map(|v| (v % image_width, v / image_width))
    .map(|(x, y)| {
      let mut rng = rand::thread_rng();
      let mut pixel_color = Color::zero();

      for _ in 0..samples_per_pixel {
        let u = (x as f32 + rng.gen::<f32>()) / (image_width as f32 - 1.0);
        let v = (y as f32 + rng.gen::<f32>()) / (image_height as f32 - 1.0);
        let r = camera.get_ray(u, v);
        pixel_color = pixel_color + trace_ray(&r, &bvh, sample_max_bounces);
      }
      pixel_color = pixel_color / (samples_per_pixel as f32);
      pixel_color = gamma_correct(pixel_color, 2.2);

      (x, y, pixel_color)
    })
    .collect();

  info!("-- Collecting output --");
  for (x, y, pixel_color) in data {
    img.put_pixel(
      x as u32,
      (image_height - y - 1) as u32,
      image::Rgb(color_f32_to_u8(pixel_color)),
    );
  }

  ///////////////////////
  // Save output
  img.save("output.png").unwrap();
  info!("-- DONE --");
}
