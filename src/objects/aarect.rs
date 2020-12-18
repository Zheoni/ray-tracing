use super::*;

const AABB_ZERO_PADDING: f64 = 0.0001;

pub enum RectAxis {
    XY,
    XZ,
    YZ,
}

pub struct Rect {
    a: usize,
    b: usize,
    c: usize,
    a0: f64,
    a1: f64,
    b0: f64,
    b1: f64,
    k: f64,
    material: Arc<dyn Material>,
}

impl Rect {
    pub fn new(
        axles: RectAxis,
        a0: f64,
        a1: f64,
        b0: f64,
        b1: f64,
        k: f64,
        material: Arc<dyn Material>,
    ) -> Self {
        use RectAxis::*;
        let (a, b, c) = match axles {
            XY => (0, 1, 2),
            XZ => (0, 2, 1),
            YZ => (1, 2, 0),
        };
        Self {
            a,
            b,
            c,
            a0,
            a1,
            b0,
            b1,
            k,
            material,
        }
    }
}

impl Hittable for Rect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.origin[self.c]) / r.direction[self.c];

        if t < t_min || t > t_max {
            return None;
        }

        let a = r.origin[self.a] + t * r.direction[self.a];
        let b = r.origin[self.b] + t * r.direction[self.b];
        if a < self.a0 || a > self.a1 || b < self.b0 || b > self.b1 {
            return None;
        }

        let u = (a - self.a0) / (self.a1 - self.a0);
        let v = (b - self.b0) / (self.b1 - self.b0);
        let outward_normal = {
            let mut v = Vec3::zero();
            v[self.c] = 1.0;
            v
        };
        let point = r.at(t);

        Some(HitRecord::new(
            r,
            t,
            u,
            v,
            point,
            outward_normal,
            self.material.as_ref(),
        ))
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        // Added padding in 3rd dimension to avoid it being 0
        let mut v_min = [0.0; 3];
        v_min[self.a] = self.a0;
        v_min[self.b] = self.b0;
        v_min[self.c] = self.k - AABB_ZERO_PADDING;
        let mut v_max = [0.0; 3];
        v_max[self.a] = self.a1;
        v_max[self.b] = self.b1;
        v_max[self.c] = self.k + AABB_ZERO_PADDING;
        Some(AABB {
            minimum: Vec3::from(v_min),
            maximum: Vec3::from(v_max),
        })
    }
}
