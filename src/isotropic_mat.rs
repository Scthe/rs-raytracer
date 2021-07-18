use std::sync::Arc;

use crate::material::{BSDFResult, Material};
use crate::ray::Ray;
use crate::texture::{SolidColorTex, Texture};
use crate::traceable::RayHit;
use crate::vec3::{Color, Vec3};

#[derive(Clone, Debug)]
/**
Isotropic := (of an object or substance) having a physical property which has the same value when measured in different directions.

Sends bounces in random direction.
*/
pub struct IsotropicMat {
  albedo: Arc<dyn Texture>,
}

impl IsotropicMat {
  #[allow(dead_code)]
  pub fn color(c: Color) -> Self {
    Self {
      albedo: Arc::new(SolidColorTex::from_color(c)),
    }
  }

  #[allow(dead_code)]
  pub fn texture(t: Arc<dyn Texture>) -> Self {
    Self { albedo: t.clone() }
  }
}

impl Material for IsotropicMat {
  fn bsdf(&self, _r_in: &Ray, hit: &RayHit) -> BSDFResult {
    BSDFResult {
      diffuse: self.albedo.sample(hit),
      bounce: Some(Ray::new(hit.p, Vec3::rand_unit())),
      ..Default::default()
    }
  }
}
