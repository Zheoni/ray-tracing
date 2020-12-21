use crate::aabb::{surrounding_box, AABB};
use crate::hittable::{HitRecord, Hittable, HittableList, Unhittable};
use crate::ray::Ray;
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::sync::Arc;

use rand::Rng;

pub struct BVHNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    b_box: AABB,
}

impl BVHNode {
    pub fn build_tree(objects: &mut [Arc<dyn Hittable>], time0: f64, time1: f64) -> Self {
        let axis: usize = rand::thread_rng().gen_range(0, 3);
        assert!(axis < 3);

        let left: Arc<dyn Hittable>;
        let right: Arc<dyn Hittable>;

        match objects.len() {
            0 => {
                left = Arc::new(Unhittable {});
                right = Arc::clone(&left);
            }
            1 => {
                left = Arc::clone(&objects[0]);
                right = Arc::clone(&objects[0]);
            }
            2 => match box_compare(objects[0].borrow(), objects[1].borrow(), axis) {
                Ordering::Less => {
                    left = Arc::clone(&objects[0]);
                    right = Arc::clone(&objects[1]);
                }
                _ => {
                    left = Arc::clone(&objects[1]);
                    right = Arc::clone(&objects[0]);
                }
            },
            _ => {
                objects.sort_by(|a, b| box_compare(a.borrow(), b.borrow(), axis));
                let mid = objects.len() / 2;
                left = Arc::new(Self::build_tree(&mut objects[..mid], time0, time1));
                right = Arc::new(Self::build_tree(&mut objects[mid..], time0, time1));
            }
        };
        let box_left = left.bounding_box(time0, time1);
        let box_right = right.bounding_box(time0, time1);

        let b_box = if let (Some(bl), Some(br)) = (box_left, box_right) {
            surrounding_box(&bl, &br)
        } else {
            AABB::default()
        };

        Self { left, right, b_box }
    }

    pub fn from_scene(mut scene: HittableList, time0: f64, time1: f64) -> Self {
        Self::build_tree(&mut scene.objects, time0, time1)
    }
}

fn box_compare(a: &dyn Hittable, b: &dyn Hittable, axis: usize) -> Ordering {
    let box_a = a.bounding_box(0.0, 0.0);
    let box_b = b.bounding_box(0.0, 0.0);

    match (box_a, box_b) {
        (Some(ba), Some(bb)) => ba.minimum[axis]
            .partial_cmp(&bb.minimum[axis])
            .unwrap_or(Ordering::Equal),
        _ => {
            eprintln!("No bounding box in bvh_node constructior.");
            Ordering::Equal
        }
    }
}

impl Hittable for BVHNode {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // If if does not hit the bounding box, return instantly
        if !self.b_box.hit(r, t_min, t_max) {
            return None;
        }

        // if it does, check for the closes hit between the left and right
        let hit_left = self.left.hit(r, t_min, t_max);
        let hit_right = self.right.hit(
            r,
            t_min,
            if let Some(ref rec) = hit_left {
                rec.t
            } else {
                t_max
            },
        );

        hit_right.or(hit_left)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        Some(self.b_box.clone())
    }
}
