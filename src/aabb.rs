use std::mem;

use crate::ray::Ray;
use crate::vec3::Point3d;

fn point_min(p0: &Point3d, p1: &Point3d) -> Point3d {
  Point3d::new(
    p0.x().min(p1.x()), //
    p0.y().min(p1.y()),
    p0.z().min(p1.z()),
  )
}

fn point_max(p0: &Point3d, p1: &Point3d) -> Point3d {
  Point3d::new(
    p0.x().max(p1.x()), //
    p0.y().max(p1.y()),
    p0.z().max(p1.z()),
  )
}

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
      min: point_min(&box0.min, &box1.min),
      max: point_max(&box0.max, &box1.max),
    }
  }

  /** Huge AABB to skip BVH and debug problems */
  pub fn ginormous() -> AABB {
    AABB {
      min: Point3d::uni(-999999999.0),
      max: Point3d::uni(999999999.0),
    }
  }

  pub fn from_point_cloud(points: &[Point3d]) -> AABB {
    if points.len() < 1 {
      panic!("Cannot create AABB from point cloud of 0 length")
    }
    let mut min = points[0].clone();
    let mut max = points[0].clone();

    for p in points {
      min = point_min(&min, &p);
      max = point_max(&max, &p);
    }

    AABB { min, max }
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

  pub fn to_points(&self) -> [Point3d; 8] {
    let min = self.min;
    let max = self.max;
    [
      Point3d::new(min.x(), min.y(), min.z()),
      Point3d::new(min.x(), min.y(), max.z()),
      Point3d::new(min.x(), max.y(), min.z()),
      Point3d::new(min.x(), max.y(), max.z()),
      Point3d::new(max.x(), max.y(), min.z()),
      Point3d::new(max.x(), max.y(), max.z()),
      Point3d::new(max.x(), min.y(), min.z()),
      Point3d::new(max.x(), min.y(), max.z()),
    ]
  }
}
