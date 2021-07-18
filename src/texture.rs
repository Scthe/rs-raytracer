use noise::{NoiseFn, Perlin};

use crate::traceable::RayHit;
use crate::vec3::Color;

/** Image-like thing that can be sampled in 2d */
pub trait Texture: Send + Sync + std::fmt::Debug {
  fn sample(&self, hit: &RayHit) -> Color;
}

///////////////////////
// SolidColorTex
#[derive(Clone, Debug)]
/** Texture that has a single color */
pub struct SolidColorTex {
  color: Color,
}

impl SolidColorTex {
  pub fn new(r: f32, g: f32, b: f32) -> SolidColorTex {
    SolidColorTex {
      color: Color::new(r, g, b),
    }
  }

  #[allow(dead_code)]
  pub fn from_color(color: Color) -> SolidColorTex {
    SolidColorTex { color }
  }
}

impl Texture for SolidColorTex {
  fn sample(&self, _hit: &RayHit) -> Color {
    self.color
  }
}

///////////////////////
// UV debug
#[derive(Clone, Debug)]
/** Texture that shows the UV coordinates */
pub struct UVDebugTex {}

impl Texture for UVDebugTex {
  fn sample(&self, hit: &RayHit) -> Color {
    Color::new(hit.u, hit.v, 0.0)
  }
}

///////////////////////
// Checker
#[derive(Clone, Debug)]
/** Black and white checker texture */
pub struct CheckerTex {
  pub color1: Color,
  pub color2: Color,
  pub scale: f32,
}

impl Texture for CheckerTex {
  fn sample(&self, hit: &RayHit) -> Color {
    let p = hit.p;
    let sines =
      (self.scale * p.x()).sin() * (self.scale * p.y()).sin() * (self.scale * p.z()).sin();
    if sines < 0.0 {
      self.color1
    } else {
      self.color2
    }
  }
}

///////////////////////
// Noise
#[derive(Debug)]
/** Random noise texture */
pub struct NoiseTex {
  // Perlin, cause noise.rs has issues with other types. Check their github..
  // Also, Perlin is easier to analyze:
  // https://github.com/3b/noise-range-test/blob/main/perlin-improved.md
  pub noise: Perlin,
  pub scale: f32,
}

impl Texture for NoiseTex {
  fn sample(&self, hit: &RayHit) -> Color {
    let scale = self.scale * 10.0; // reasonable defaults?
    let v = self
      .noise
      .get([(hit.u * scale) as f64, (hit.v * scale) as f64, 0.0]);
    Color::uni(v as f32 * 0.5 + 0.5)
  }
}

///////////////////////
// Image
use image::io::Reader as ImageReader;
use std::path::Path;

// TODO add support for alpha channel
#[derive(Debug)]
/** Image from file */
pub struct ImageTex {
  image: image::RgbaImage,
}

impl ImageTex {
  pub fn new(path: &Path) -> ImageTex {
    let image = ImageReader::open(path).unwrap();
    ImageTex {
      image: image.decode().unwrap().to_rgba8(),
    }
  }
}

impl Texture for ImageTex {
  fn sample(&self, hit: &RayHit) -> Color {
    let x = self.image.width() as f32 * hit.u.clamp(0.0, 0.99999);
    let y = self.image.height() as f32 * hit.v.clamp(0.0, 0.99999);
    let p = self.image.get_pixel(x as u32, y as u32);
    Color::new(
      p[0] as f32 / 255.0,
      p[1] as f32 / 255.0,
      p[2] as f32 / 255.0,
    )
  }
}
