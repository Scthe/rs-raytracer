use std::mem;

use crate::ray::Ray;
use crate::vec3::Point3d;

/**
 * Axis-aligned bounding rectangular parallelepiped
 * ...
 * ...
 * just call it
 * `Axis Aligned Bounding Box` - AABB
 */
#[derive(Clone, Debug, Copy)]
pub struct AABB {
  pub min: Point3d,
  pub max: Point3d,
}

impl AABB {
  pub fn merge(box0: &AABB, box1: &AABB) -> AABB {
    AABB {
      min: Point3d::new(
        box0.min.x().min(box1.min.x()),
        box0.min.y().min(box1.min.y()),
        box0.min.z().min(box1.min.z()),
      ),
      max: Point3d::new(
        box0.max.x().max(box1.max.x()),
        box0.max.y().max(box1.max.y()),
        box0.max.z().max(box1.max.z()),
      ),
    }
  }

  pub fn check_intersection(&self, r: &Ray, t_min_: f32, t_max_: f32) -> bool {
    let mut t_min = t_min_;
    let mut t_max = t_max_;

    for axis_idx in 0..3 {
      // slab algoritm:
      // solve for t: `A + tb = x0` and `A + tb = x1`
      // where x0 and x1 are point of cross of ray and axis-parallel lines
      //  $ t0 = (x0 - A) / b , similar for t1
      let inv_b = 1.0 / r.dir[axis_idx];
      let mut t0 = (self.min[axis_idx] - r.origin[axis_idx]) * inv_b;
      let mut t1 = (self.max[axis_idx] - r.origin[axis_idx]) * inv_b;
      // check 'reverse' direction
      if r.dir[axis_idx] < 0.0 {
        mem::swap(&mut t0, &mut t1);
      }

      // With each axis that we iter through we need to find
      // a common segment of the ray that is 'inside' our AABB.
      // And with each iter our segment (between t0_min, t0_max)
      // is getting smaller.
      t_min = t0.max(t_min);
      t_max = t1.min(t_max);
      // Check if ray segment is still valid
      if t_max <= t_min {
        return false;
      }
    }

    true
  }
}
