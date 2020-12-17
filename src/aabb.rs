use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Clone)]
pub struct AABB {
    pub minimum: Vec3,
    pub maximum: Vec3,
}

impl AABB {
    // //default
    // pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
    //     for a in 0..3 {
    //         let with_min = (self.minimum[a] - r.origin[a]) / r.direction[a];
    //         let with_max = (self.maximum[a] - r.origin[a]) / r.direction[a];
    //         let t0 = f64::min(with_min, with_max);
    //         let t1 = f64::max(with_min, with_max);
    //         let t_min = t_min.max(t0);
    //         let t_max = t_max.min(t1);
    //         if t_max <= t_min {
    //             return false;
    //         }
    //     }
    //     true
    // }

    // //pixar
    // pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
    //     for a in 0..3 {
    //         let invD = r.direction[a].recip();
    //         let (t0, t1) = if invD >= 0.0 {
    //             let t0 = (self.minimum[a] - r.origin[a]) * invD;
    //             let t1 = (self.maximum[a] - r.origin[a]) * invD;
    //             (t0, t1)
    //         } else {
    //             let t0 = (self.minimum[a] - r.origin[a]) * invD;
    //             let t1 = (self.maximum[a] - r.origin[a]) * invD;
    //             (t0, t1)
    //         };

    //         let t_min = t_min.max(t0);
    //         let t_max = t_max.min(t1);
    //         if t_max <= t_min {
    //             return false;
    //         }
    //     }
    //     true
    // }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        for a in 0..3 {
            let inv_d = r.direction[a].recip();
            let mut t0 = (self.minimum[a] - r.origin[a]) * inv_d;
            let mut t1 = (self.maximum[a] - r.origin[a]) * inv_d;

            if inv_d < 0.0 {
                let temp = t0;
                t0 = t1;
                t1 = temp;
            }

            let t_min = t_min.max(t0);
            let t_max = t_max.min(t1);
            if t_max <= t_min {
                return false;
            }
        }
        true
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
