use rand::Rng;
use std::sync::Arc;

use crate::aabb::AABB;
use crate::ray::Ray;
use crate::traceable::{RayHit, Traceable};
use crate::vec3::Vec3;
use crate::world::{World, WorldObjectsList};

/** Bounding Volume Hierarchy node */
pub struct BVHNode {
  aabb: AABB,
  child_left: Arc<dyn Traceable>,
  child_right: Arc<dyn Traceable>,
}

impl BVHNode {
  pub fn build(world: &World) -> BVHNode {
    BVHNode::build_impl(&world.objects, 0, world.objects.len())
  }

  fn build_impl(world_objects: &WorldObjectsList, start_idx: usize, end_idx: usize) -> BVHNode {
    let mut node = BVHNode::subdivide_into_bvh(world_objects, start_idx, end_idx);

    // recalc aabb
    match (
      node.child_left.bounding_box(),
      node.child_right.bounding_box(),
    ) {
      (Some(bb_left), Some(bb_right)) => {
        node.aabb = AABB::merge(&bb_left, &bb_right);
      }
      _ => {
        panic!("Tried to create BVH, but some objects do not have bounding box");
      }
    };

    node
  }

  fn subdivide_into_bvh(
    world_objects: &WorldObjectsList,
    start_idx: usize,
    end_idx: usize,
  ) -> BVHNode {
    let objects_count = end_idx - start_idx;
    let mock_aabb = AABB {
      min: Vec3::zero(),
      max: Vec3::one(),
    };

    if objects_count < 1 {
      panic!("Tried to create BVHNode from empty objects list");
    } else if objects_count == 1 {
      return BVHNode {
        aabb: mock_aabb,
        child_left: world_objects[start_idx].clone(),
        child_right: world_objects[start_idx].clone(),
      };
    } else if objects_count == 2 {
      return BVHNode {
        aabb: mock_aabb,
        child_left: world_objects[start_idx].clone(),
        child_right: world_objects[start_idx + 1].clone(),
      };
    }

    let mut objects_copy = world_objects[start_idx..end_idx].to_vec();
    let axis_to_sort_by: u32 = rand::thread_rng().gen_range(0..3);
    sort_by_axis_distance(&mut objects_copy, axis_to_sort_by as usize);

    let mid = objects_copy.len() / 2;
    BVHNode {
      aabb: mock_aabb,
      child_left: Arc::new(BVHNode::build_impl(&objects_copy, 0, mid)),
      child_right: Arc::new(BVHNode::build_impl(&objects_copy, mid, objects_copy.len())),
    }
  }
}

fn sort_by_axis_distance(objects: &mut WorldObjectsList, axis_to_sort_by: usize) {
  objects.sort_by(|a, b| match (a.bounding_box(), b.bounding_box()) {
    (Some(bb_a), Some(bb_b)) => {
      let val_a = bb_a.min[axis_to_sort_by];
      let val_b = bb_b.min[axis_to_sort_by];
      val_a.partial_cmp(&val_b).unwrap()
    }
    _ => {
      panic!("Tried to create BVH, but some objects do not have bounding box");
    }
  });
}

impl Traceable for BVHNode {
  fn bounding_box(&self) -> Option<AABB> {
    Some(self.aabb)
  }

  fn check_intersection(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<RayHit> {
    if !self.aabb.check_intersection(r, t_min, t_max) {
      // this node was not hit by the ray, skip entirely
      return None;
    }

    match self.child_left.check_intersection(r, t_min, t_max) {
      None => {
        // left missed, return right that maybe hit
        return self.child_right.check_intersection(r, t_min, t_max);
      }
      Some(left_hit_data) => {
        // check if right hit closer than left
        let hit_right = self
          .child_right
          .check_intersection(r, t_min, left_hit_data.t);
        match hit_right {
          None => return Some(left_hit_data), // left hit, right missed (or was farther)
          Some(right_hit_data) => return Some(right_hit_data), // both hit, but right was closer
        }
      }
    }
  }
}
