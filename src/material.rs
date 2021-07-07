use std::fmt;

use crate::ray::Ray;
use crate::traceable::RayHit;
use crate::utils::{clamp, reflect, reflectance_schlick, refract};
use crate::vec3::{Color, Vec3};

const IOR_AIR: f32 = 1.0; // blah, blah, vacuum, blah, blah

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
    let reflected = reflect(r_in.dir, hit.normal);
    let roughness_scatter = Vec3::rand_unit() * clamp(self.roughness, 0.0, 1.0);
    *scattered = Ray::new(hit.p, reflected + roughness_scatter);
    *attenuation = self.albedo;
    scattered.dir.dot(hit.normal) > 0.0
  }
}

// Why is glass called dielectric?! I'm following the book here, but this
// is the least interesting thing about dielectrics TBH.
#[derive(Clone, Debug)]
pub struct Dielectric {
  pub ior: f32, // https://en.wikipedia.org/wiki/List_of_refractive_indices
}

impl Material for Dielectric {
  fn scatter(
    &self,
    r_in: &Ray,
    hit: &RayHit,
    attenuation: &mut Color,
    scattered: &mut Ray,
  ) -> bool {
    *attenuation = Color::one();

    let (ior_from, ior_into) = if hit.front_face {
      (IOR_AIR, self.ior)
    } else {
      (self.ior, IOR_AIR)
    };

    let reflectance_at_angle = reflectance_schlick(r_in.dir, hit.normal, ior_from, ior_into);
    // I don't like this randomness here, but let's do by the book for now..
    let sample_use_reflect_cause_angle = reflectance_at_angle < rand::random::<f32>();
    let maybe_refracted = refract(r_in.dir, hit.normal, ior_from, ior_into);

    let refracted = match maybe_refracted {
      Some(x) if !sample_use_reflect_cause_angle => x,
      _ => reflect(r_in.dir, hit.normal),
    };

    *scattered = Ray::new(hit.p, refracted);
    true
  }
}
