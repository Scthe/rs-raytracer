use std::fmt;

use crate::ray::Ray;
use crate::traceable::RayHit;
use crate::utils::{clamp, reflect};
use crate::vec3::{Color, Vec3};

pub trait Material: fmt::Debug {
  fn scatter(&self, r_in: &Ray, hit: &RayHit, attenuation: &mut Color, scattered: &mut Ray)
    -> bool;
}

#[derive(Clone, Debug)]
pub struct SolidColor {
  pub color: Color,
}

impl Material for SolidColor {
  fn scatter(
    &self,
    r_in: &Ray,
    hit: &RayHit,
    attenuation: &mut Color,
    scattered: &mut Ray,
  ) -> bool {
    *attenuation = self.color;
    true
  }
}

#[derive(Clone, Debug)]
pub struct Lambert {
  pub albedo: Color,
}

impl Material for Lambert {
  fn scatter(
    &self,
    r_in: &Ray,
    hit: &RayHit,
    attenuation: &mut Color,
    scattered: &mut Ray,
  ) -> bool {
    let mut scatter_direction = hit.normal + Vec3::rand_unit();
    if scatter_direction.near_zero() {
      scatter_direction = hit.normal;
    }

    *scattered = Ray::new(hit.p, scatter_direction);
    *attenuation = self.albedo;
    true
  }
}

#[derive(Clone, Debug)]
pub struct Metal {
  pub albedo: Color,
  pub roughness: f32,
}

impl Material for Metal {
  fn scatter(
    &self,
    r_in: &Ray,
    hit: &RayHit,
    attenuation: &mut Color,
    scattered: &mut Ray,
  ) -> bool {
    let reflected = reflect((r_in.dir).unit_vector(), hit.normal);
    let roughness_scatter = Vec3::rand_unit() * clamp(self.roughness, 0.0, 1.0);
    *scattered = Ray::new(hit.p, reflected + roughness_scatter);
    *attenuation = self.albedo;
    scattered.dir.dot(hit.normal) > 0.0
  }
}
