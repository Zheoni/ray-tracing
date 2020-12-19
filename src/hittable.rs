use crate::aabb::{surrounding_box, AABB};
use crate::material::Material;
use crate::ray::Ray;
use std::sync::Arc;
use vec3::Vec3;

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB>;
}

pub struct Unhittable {}
impl Hittable for Unhittable {
    fn hit(&self, _r: &Ray, _t_min: f64, _t_max: f64) -> Option<HitRecord> {
        None
    }
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        None
    }
}

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
    /// always points against the ray and [HitRecord.front_face]
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

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = r.direction.dot(&outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
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

pub struct Translate {
    object: Arc<dyn Hittable>,
    offset: Vec3,
}

impl Translate {
    pub fn new(object: Arc<dyn Hittable>, offset: Vec3) -> Self {
        Self { object, offset }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved_r = Ray::new(r.origin - self.offset, r.direction, r.time);

        if let Some(mut hit) = self.object.hit(&moved_r, t_min, t_max) {
            hit.point += self.offset;
            hit.set_face_normal(&moved_r, hit.normal);
            Some(hit)
        } else {
            None
        }
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        if let Some(mut bb) = self.object.bounding_box(time0, time1) {
            bb.minimum += self.offset;
            bb.maximum += self.offset;
            Some(bb)
        } else {
            None
        }
    }
}

pub struct RotateY {
    object: Arc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Option<AABB>,
}

impl RotateY {
    pub fn new(object: Arc<dyn Hittable>, angle: f64) -> Self {
        let angle = angle.to_radians();
        let sin_theta = angle.sin();
        let cos_theta = angle.cos();

        let bbox = if let Some(bb) = object.bounding_box(0.0, 1.0) {
            let mut min = Vec3::splat(f64::INFINITY);
            let mut max = Vec3::splat(f64::NEG_INFINITY);

            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        let (i, j, k) = (i as f64, j as f64, k as f64);

                        let x = i * bb.maximum.x() + (1.0 - i) * bb.minimum.x();
                        let y = j * bb.maximum.y() + (1.0 - j) * bb.minimum.y();
                        let z = k * bb.maximum.z() + (1.0 - k) * bb.minimum.z();

                        let newx = cos_theta * x + sin_theta * z;
                        let newz = -sin_theta * x + cos_theta * z;

                        let tester = Vec3::new(newx, y, newz);
                        for c in 0..3 {
                            min[c] = min[c].min(tester[c]);
                            max[c] = max[c].max(tester[c]);
                        }
                    }
                }
            }
            Some(AABB {
                minimum: min,
                maximum: max,
            })
        } else {
            None
        };

        Self {
            object,
            sin_theta,
            cos_theta,
            bbox,
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut origin = r.origin;
        origin[0] = self.cos_theta * r.origin[0] - self.sin_theta * r.origin[2];
        origin[2] = self.sin_theta * r.origin[0] + self.cos_theta * r.origin[2];

        let mut direction = r.direction;
        direction[0] = self.cos_theta * r.direction[0] - self.sin_theta * r.direction[2];
        direction[2] = self.sin_theta * r.direction[0] + self.cos_theta * r.direction[2];

        let rotated_r = Ray::new(origin, direction, r.time);

        if let Some(mut hit) = self.object.hit(&rotated_r, t_min, t_max) {
            let mut point = hit.point;
            point[0] = self.cos_theta * hit.point[0] + self.sin_theta * hit.point[2];
            point[2] = -self.sin_theta * hit.point[0] + self.cos_theta * hit.point[2];

            let mut normal = hit.normal;
            normal[0] = self.cos_theta * hit.normal[0] + self.sin_theta * hit.normal[2];
            normal[2] = -self.sin_theta * hit.normal[0] + self.cos_theta * hit.normal[2];

            hit.point = point;
            hit.normal = normal;

            Some(hit)
        } else {
            None
        }
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        self.bbox.clone()
    }
}
