use super::*;

pub struct Translate<H: Hittable> {
    object: H,
    offset: Vec3,
}

impl<H: Hittable> Translate<H> {
    pub fn new(object: H, offset: Vec3) -> Self {
        Self { object, offset }
    }
}

impl<H: Hittable> Hittable for Translate<H> {
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
