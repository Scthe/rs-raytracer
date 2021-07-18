use std::sync::Arc;

use crate::material::{BSDFResult, Material};
use crate::ray::Ray;
use crate::texture::{SolidColorTex, Texture};
use crate::traceable::RayHit;
use crate::vec3::Color;

#[derive(Clone, Debug)]
pub struct DiffuseLight {
  albedo: Arc<dyn Texture>,
  strength: f32,
}

impl DiffuseLight {
  #[allow(dead_code)]
  pub fn color(c: Color, strength: f32) -> Self {
    Self {
      albedo: Arc::new(SolidColorTex::from_color(c)),
      strength,
    }
  }

  #[allow(dead_code)]
  pub fn texture(t: Arc<dyn Texture>, strength: f32) -> Self {
    Self {
      albedo: t.clone(),
      strength,
    }
  }
}

impl Material for DiffuseLight {
  fn bsdf(&self, _r_in: &Ray, hit: &RayHit) -> BSDFResult {
    let color = self.albedo.sample(hit);

    BSDFResult {
      diffuse: color,
      emissive: color * self.strength,
      bounce: None, // important
    }
  }
}
