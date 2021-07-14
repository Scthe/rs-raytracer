use std::sync::Arc;

use rand::Rng;

use crate::aabb::AABB;
use crate::isotropic_mat::IsotropicMat;
use crate::material::Material;
use crate::ray::Ray;
use crate::texture::Texture;
use crate::traceable::{RayHit, Traceable};
use crate::vec3::{Color, Vec3};

/** ATM it's just a plane in xy space */
#[derive(Clone)]
pub struct Volumetric {
  /** Not physic based, just some value. No value range either */
  pub density: f32,
  /** Material that 'affects' the ray when it traverses this volume */
  pub phase_function: Arc<dyn Material>,
  /** Shape of the volumetric expressed as some other shape */
  pub shape: Arc<dyn Traceable>,
}

impl Volumetric {
  #[allow(dead_code)]
  pub fn color(shape: Arc<dyn Traceable>, density: f32, color: Color) -> Self {
    Self {
      density,
      shape: shape.clone(),
      phase_function: Arc::new(IsotropicMat::color(color)),
    }
  }

  #[allow(dead_code)]
  pub fn texture(shape: Arc<dyn Traceable>, density: f32, t: Arc<dyn Texture>) -> Self {
    Self {
      density,
      shape: shape.clone(),
      phase_function: Arc::new(IsotropicMat::texture(t)),
    }
  }
}

impl Traceable for Volumetric {
  fn bounding_box(&self) -> Option<AABB> {
    self.shape.bounding_box()
  }

  fn check_intersection(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<RayHit> {
    // Check where in global space we pass the volume. We ignore t_min, t_max,
    // just whan to know if ray EVER is inside volume.
    // We assume shape is convex, so there is only one pair of entry-exit points
    let maybe_hit1 = self
      .shape
      .check_intersection(r, -f32::INFINITY, f32::INFINITY);
    let maybe_both_hits = maybe_hit1
      .map(|hit0| {
        let maybe_hit2 = self
          .shape
          .check_intersection(r, hit0.t + 0.0001, f32::INFINITY);
        maybe_hit2.map(|hit1| (hit0, hit1))
      })
      .flatten();

    match maybe_both_hits {
      Some((mut hit0, mut hit1)) => {
        hit0.t = hit0.t.max(t_min);
        hit1.t = hit1.t.min(t_max);
        if hit0.t >= hit1.t {
          return None;
        }

        // do not go backward from ray origin
        hit0.t = hit0.t.max(0.0);
        let ray_length = r.dir.length(); // e.g. transform does not normalize direction. Tho usually 1.0
        let distance_inside_boundary = (hit1.t - hit0.t) * ray_length;
        // we probabilistically bail out. This is not physics based, density is just 'some' value
        let mut rng = rand::thread_rng();
        let bounce_prob: f32 = rng.gen();
        let hit_distance = bounce_prob / self.density;
        if hit_distance > distance_inside_boundary {
          return None;
        }

        let t = hit0.t + hit_distance / ray_length;
        Some(RayHit {
          p: r.at(t),
          t,
          u: hit0.u,
          v: hit0.v,
          material: self.phase_function.clone(),
          // Volume describes smth. e.g. like particles suspended in the air. The ray bounces
          // will rarely be in the same direction. TBH there are a few probability functions
          // that describe this behavour, like Rayleigh scattering. It all depends e.g. on
          // wavelength of light ray etc.
          //
          // Instead, our IsotropicMat material will ignore normal and front_face
          // and pick bounce randomly.
          normal: Vec3::rand_unit(), // from book: arbitrary.
          front_face: true,          // from book: also arbitrary
        })
      }
      _ => None,
    }
  }
}
