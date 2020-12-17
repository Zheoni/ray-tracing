use crate::aabb::{surrounding_box, AABB};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::sync::Arc;

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB>;
}

pub struct HitRecord<'a> {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: &'a dyn Material,
}

impl<'a> HitRecord<'a> {
    /// Creates a new [HitRecord] where the normal will
    /// always points against the ray and [HitRecord.front_face]
    /// tell us if the normal is points inwards or outwards the object.
    pub fn new(
        r: &Ray,
        t: f64,
        point: Vec3,
        outward_normal: Vec3,
        material: &'a dyn Material,
    ) -> Self {
        let front_face = r.direction.dot(&outward_normal) < 0.0;
        // The normal against the ray
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        HitRecord {
            point,
            normal,
            t,
            front_face,
            material,
        }
    }
}

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut rec: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if let Some(hit) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = hit.t;
                rec = Some(hit);
            }
        }

        rec
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        if self.objects.is_empty() {
            None
        } else {
            let mut temp_box: AABB = self.objects[0].bounding_box(time0, time1)?;
            for object in &self.objects[1..] {
                if let Some(object_box) = object.bounding_box(time0, time1) {
                    temp_box = surrounding_box(&temp_box, &object_box);
                } else {
                    return None;
                }
            }
            Some(temp_box)
        }
    }
}
