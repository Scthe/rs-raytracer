use crate::vec3::{Color, Point3d, Vec3};

/// change number in -1..1 range to 0..1
pub fn to_0_1(v: f32) -> f32 {
  0.5 * (v + 1.0)
}

pub fn lerp_vec3(a: Vec3, b: Vec3, t: f32) -> Vec3 {
  a * (1.0 - t) + b * t
}

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
