use super::*;

pub struct RotateY<H: Hittable> {
    object: H,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Option<AABB>,
}

impl<H: Hittable> RotateY<H> {
    pub fn new(object: H, angle: f64) -> Self {
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

impl<H: Hittable> Hittable for RotateY<H> {
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
