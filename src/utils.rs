use crate::vec3::{Color, Point3d, Vec3};
use rand::Rng;

/// change number in -1..1 range to 0..1
pub fn to_0_1(v: f32) -> f32 {
  0.5 * (v + 1.0)
}

pub fn lerp_vec3(a: Vec3, b: Vec3, t: f32) -> Vec3 {
  a * (1.0 - t) + b * t
}

pub fn color_f32_to_u8(col: Color) -> [u8; 3] {
  [
    (col[0] * 255.999).clamp(0.0, 255.0) as u8,
    (col[1] * 255.999).clamp(0.0, 255.0) as u8,
    (col[2] * 255.999).clamp(0.0, 255.0) as u8,
  ]
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
  v - n * (2.0 * v.dot(n))
}

pub fn refract(light_dir: Vec3, n: Vec3, ior_from: f32, ior_into: f32) -> Option<Vec3> {
  let etai_over_etat = ior_from / ior_into;
  let cos_theta = (!light_dir).dot(n).min(1.0);
  let sin_theta = (1.0 - cos_theta * cos_theta).sqrt(); // from Pythagorean identities

  // If in Snell's law  we have
  //   $ sin(θ′) = (η / η′) * sin(θ) , where η′ are IORs
  // we can have in some cases e.g.
  //   $ sin(θ′) = 1.5 * sin(θ)
  // which for some values of θ has no solution.
  // e.g. when θ=1 the right side is 1.5 and sin(θ′) will never have this value
  let cannot_refract = etai_over_etat * sin_theta > 1.0;
  if cannot_refract {
    return None;
  }

  let r_out_perp = (light_dir + n * cos_theta) * etai_over_etat;
  let r_out_parallel = n * -1.0 * ((1.0 - r_out_perp.length_squared()).abs()).sqrt();
  Some(r_out_perp + r_out_parallel)
}

// Schlick Approximation for reflection at angle
pub fn reflectance_schlick(light_dir: Vec3, n: Vec3, ior_from: f32, ior_into: f32) -> f32 {
  let cos_theta = (!light_dir).dot(n).min(1.0);
  let mut r0 = (ior_from - ior_into) / (ior_from + ior_into);
  r0 = r0 * r0;
  r0 + (1.0 - r0) * ((1.0 - cos_theta).powf(5.0))
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

// https://mathworld.wolfram.com/DiskPointPicking.html
pub fn random_in_disk(r: f32) -> Point3d {
  let r_sqrt = r.sqrt();
  let mut rng = rand::thread_rng();
  let theta = rng.gen_range(0.0..(2.0 * std::f32::consts::PI));

  Point3d::new(r_sqrt * theta.cos(), r_sqrt * theta.sin(), 0.0)
}

pub fn gamma_correct(col: Color, gamma: f32) -> Color {
  Color::new(
    col.x().powf(1.0 / gamma),
    col.y().powf(1.0 / gamma),
    col.z().powf(1.0 / gamma),
  )
}
