use crate::aabb::{surrounding_box, AABB};
use crate::material::Material;
use crate::ray::Ray;
use vec3::Vec3;

/// Allows a struct to interact with light; being hitted by
/// a ray
pub trait Hittable: Send + Sync {
    /// Checks if the ray hits in the given time the struct and if so returns a
    /// [Some] value with a [HitRecord], else return [None]
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB>;
}

/// Struct that implements [Hittable] but is never hittet neither it has a bounding box
pub struct Unhittable;
impl Hittable for Unhittable {
    fn hit(&self, _r: &Ray, _t_min: f64, _t_max: f64) -> Option<HitRecord> {
        None
    }
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        None
    }
}

/// Data returned from a ray hit into a [Hittable]
pub struct HitRecord<'a> {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
    pub material: &'a dyn Material,
}

impl<'a> HitRecord<'a> {
    /// Creates a new [HitRecord] where the normal will
    /// always points against the ray and [front_face](HitRecord::front_face)
    /// tell us if the normal is points inwards or outwards the object.
    pub fn new(
        r: &Ray,
        t: f64,
        u: f64,
        v: f64,
        point: Vec3,
        outward_normal: Vec3,
        material: &'a dyn Material,
    ) -> Self {
        let mut hr = Self {
            point,
            normal: Vec3::zero(),
            t,
            u,
            v,
            front_face: false,
            material,
        };

        hr.set_face_normal(r, outward_normal);
        hr
    }

    /// Sets the [front_face](HitRecord::front_face) and [normal][HitRecord::normal]
    /// calculating it from the given ray and outward normal (outward from the object)
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = r.direction.dot(&outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

/// List of hittables that implements [Hittable] too
pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl Hittable for HittableList {
    /// Returns the hit of the closer object. Tests the hit for every object in the
    /// list.
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

    /// Returns the bounding box of all the elements of the list
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
