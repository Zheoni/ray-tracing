use crate::ray::Ray;
use vec3::Vec3;

#[derive(Clone, Default)]
pub struct AABB {
    pub minimum: Vec3,
    pub maximum: Vec3,
}

impl AABB {
    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        let inv_d = r.direction.map(f64::recip);
        let t0 = (self.minimum - r.origin) * inv_d;
        let t1 = (self.maximum - r.origin) * inv_d;

        let (t0, t1) = (
            inv_d.zip_with3(t0, t1, |x, a, b| if x < 0.0 { b } else { a }),
            inv_d.zip_with3(t0, t1, |x, a, b| if x < 0.0 { a } else { b }),
        );

        let start = t_min.max(t0.reduce(f64::max));
        let end = t_max.min(t1.reduce(f64::min));
        end > start
    }
}

#[must_use]
pub fn surrounding_box(box0: &AABB, box1: &AABB) -> AABB {
    let small = Vec3::new(
        box0.minimum.x().min(box1.minimum.x()),
        box0.minimum.y().min(box1.minimum.y()),
        box0.minimum.z().min(box1.minimum.z()),
    );
    let big = Vec3::new(
        box0.maximum.x().max(box1.maximum.x()),
        box0.maximum.y().max(box1.maximum.y()),
        box0.maximum.z().max(box1.maximum.z()),
    );
    AABB {
        minimum: small,
        maximum: big,
    }
}
