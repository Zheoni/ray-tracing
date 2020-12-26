use crate::aabb::{surrounding_box, AABB};
use crate::hittable::{HitRecord, Hittable, HittableList, Unhittable};
use crate::ray::Ray;
use std::borrow::Borrow;
use std::cmp::Ordering;

use rand::Rng;

pub struct BVH {
    node: BVHNode,
    b_box: AABB,
}

pub enum BVHNode {
    Node {
        left: Box<BVH>,
        right: Box<BVH>,
    },
    Leaf(Box<dyn Hittable>),
}

impl BVH {
    pub fn build_tree(mut objects: Vec<Box<dyn Hittable>>, time0: f64, time1: f64) -> Self {
        use BVHNode::*;
        let axis: usize = rand::thread_rng().gen_range(0, 3);
        assert!(axis < 3);

        match objects.len() {
            0 => Self {
                node: Leaf(Box::new(Unhittable {})),
                b_box: AABB::default(),
            },
            1 => Self {
                b_box: objects[0].bounding_box(time0, time1).unwrap_or_default(),
                node: Leaf(objects.pop().unwrap()),
            },
            _ => {
                objects.sort_by(|a, b| box_compare(a.borrow(), b.borrow(), axis));
                let mid = objects.len() / 2;

                let right = Box::new(Self::build_tree(
                    objects.drain(mid..).collect(),
                    time0,
                    time1,
                ));
                let left = Box::new(Self::build_tree(objects, time0, time1));

                let box_left = left.bounding_box(time0, time1);
                let box_right = right.bounding_box(time0, time1);

                let b_box = if let (Some(bl), Some(br)) = (box_left, box_right) {
                    surrounding_box(&bl, &br)
                } else {
                    AABB::default()
                };

                Self {
                    node: Node { left, right },
                    b_box,
                }
            }
        }
    }

    pub fn from_scene(scene: HittableList, time0: f64, time1: f64) -> Self {
        Self::build_tree(scene.objects, time0, time1)
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

impl Hittable for BVH {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        use BVHNode::*;
        // If if does not hit the bounding box, return instantly
        if !self.b_box.hit(r, t_min, t_max) {
            return None;
        }

        // if it does, check for the closes hit between the left and right
        match &self.node {
            Leaf(leaf) => leaf.hit(r, t_min, t_max),
            Node { left, right } => {
                let hit_left = left.hit(r, t_min, t_max);
                let hit_right = right.hit(
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
        }
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        Some(self.b_box.clone())
    }
}
