use crate::vec3::{Color, Point3d, Vec3};
use rand::{thread_rng, Rng};

/// change number in -1..1 range to 0..1
pub fn to_0_1(v: f32) -> f32 {
  0.5 * (v + 1.0)
}

pub fn lerp_vec3(a: Vec3, b: Vec3, t: f32) -> Vec3 {
  a * (1.0 - t) + b * t
}

// TODO update Rust to 1.50 and this is in stable
pub fn clamp(x: f32, min_val: f32, max_val: f32) -> f32 {
  min_val.max(max_val.min(x))
}

pub fn color_f32_to_u8(col: Color) -> [u8; 3] {
  [
    clamp(col[0] * 255.999, 0.0, 255.0) as u8,
    clamp(col[1] * 255.999, 0.0, 255.0) as u8,
    clamp(col[2] * 255.999, 0.0, 255.0) as u8,
  ]
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
  v - n * (2.0 * v.dot(n))
}

pub fn random_in_unit_sphere() -> Point3d {
  let mut rng = rand::thread_rng();
  let u = rng.gen_range(-1.0..=1.0);
  let theta = rng.gen_range(0.0..(2.0 * std::f32::consts::PI));

  Point3d::new(
    (1.0f32 - u * u).sqrt() * theta.cos(),
    (1.0f32 - u * u).sqrt() * theta.sin(),
    u,
  )
}

pub fn gamma_correct(col: Color, gamma: f32) -> Color {
  Color::new(
    col.x().powf(1.0 / gamma),
    col.y().powf(1.0 / gamma),
    col.z().powf(1.0 / gamma),
  )
}
