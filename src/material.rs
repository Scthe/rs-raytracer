use std::fmt;
use std::sync::Arc;

use crate::ray::Ray;
use crate::texture::{SolidColorTex, Texture};
use crate::traceable::RayHit;
use crate::utils::{reflect, reflectance_schlick, refract};
use crate::vec3::{Color, Vec3};

const IOR_AIR: f32 = 1.0; // blah, blah, vacuum, blah, blah

pub struct BSDFResult {
  pub diffuse: Color,
  pub emissive: Color,
  pub bounce: Option<Ray>,
}

impl BSDFResult {
  pub fn with_normal_bounce(&mut self, hit: &RayHit) -> &mut Self {
    self.bounce = Some(Ray::new(hit.p, hit.normal));
    self
  }
}

impl Default for BSDFResult {
  fn default() -> Self {
    BSDFResult {
      diffuse: Color::zero(),
      emissive: Color::zero(),
      bounce: None,
    }
  }
}

///////////////////////
// Material
pub trait Material: fmt::Debug + Send + Sync {
  fn bsdf(&self, r_in: &Ray, hit: &RayHit) -> BSDFResult;
}

///////////////////////
// Solid color
#[derive(Clone, Debug)]
pub struct SolidColor {
  pub color: Color,
}

impl Material for SolidColor {
  fn bsdf(&self, _r_in: &Ray, hit: &RayHit) -> BSDFResult {
    let mut result = BSDFResult {
      diffuse: self.color,
      ..Default::default()
    };
    result.with_normal_bounce(hit);
    result
  }
}

///////////////////////
// Lambert
#[derive(Clone, Debug)]
pub struct Lambert {
  albedo: Arc<dyn Texture>,
}

impl Lambert {
  pub fn color(r: f32, g: f32, b: f32) -> Self {
    Self {
      albedo: Arc::new(SolidColorTex::new(r, g, b)),
    }
  }

  pub fn from_color(c: Color) -> Self {
    Self {
      albedo: Arc::new(SolidColorTex::from_color(c)),
    }
  }

  pub fn texture(t: Arc<dyn Texture>) -> Self {
    Self { albedo: t.clone() }
  }
}

impl Material for Lambert {
  fn bsdf(&self, _r_in: &Ray, hit: &RayHit) -> BSDFResult {
    let mut scatter_direction = hit.normal + Vec3::rand_unit();
    if scatter_direction.near_zero() {
      scatter_direction = hit.normal;
    }

    BSDFResult {
      diffuse: self.albedo.sample(hit),
      bounce: Some(Ray::new(hit.p, scatter_direction)),
      ..Default::default()
    }
  }
}

///////////////////////
// Metal
#[derive(Clone, Debug)]
pub struct Metal {
  pub albedo: Color,
  pub roughness: f32,
}

impl Material for Metal {
  fn bsdf(&self, r_in: &Ray, hit: &RayHit) -> BSDFResult {
    let reflected = reflect(r_in.dir, hit.normal);
    let roughness_scatter = Vec3::rand_unit() * self.roughness.clamp(0.0, 1.0);
    let scattered = Ray::new(hit.p, reflected + roughness_scatter);

    let mut result = BSDFResult {
      diffuse: self.albedo,
      ..Default::default()
    };
    if scattered.dir.dot(hit.normal) > 0.0 {
      result.bounce = Some(scattered)
    }
    result
  }
}

///////////////////////
// Dielectric

// Why is glass called dielectric?! I'm following the book here, but this
// is the least interesting thing about dielectrics TBH.
#[derive(Clone, Debug)]
pub struct Dielectric {
  pub albedo: Color,
  pub ior: f32, // https://en.wikipedia.org/wiki/List_of_refractive_indices
}

impl Material for Dielectric {
  fn bsdf(&self, r_in: &Ray, hit: &RayHit) -> BSDFResult {
    let (ior_from, ior_into) = if hit.front_face {
      (IOR_AIR, self.ior)
    } else {
      (self.ior, IOR_AIR)
    };

    let reflectance_at_angle = reflectance_schlick(r_in.dir, hit.normal, ior_from, ior_into);
    // I don't like this randomness here, but let's do by the book for now..
    let sample_use_reflect_cause_angle = reflectance_at_angle > rand::random::<f32>();
    let maybe_refracted = refract(r_in.dir, hit.normal, ior_from, ior_into);

    let refracted = match maybe_refracted {
      Some(x) if !sample_use_reflect_cause_angle => x,
      _ => reflect(r_in.dir, hit.normal),
    };

    BSDFResult {
      diffuse: self.albedo,
      bounce: Some(Ray::new(hit.p, refracted)),
      ..Default::default()
    }
  }
}
