use super::*;

const AABB_ZERO_PADDING: f64 = 0.0001;

/// Defines the axles a [Rect] is aligned to
pub trait RectAxis: Send + Sync {
    const AXIS: Axis;
    const OTHER1: Axis;
    const OTHER2: Axis;
}

/// Axis aligned rectangle
#[derive(Clone)]
pub struct Rect<A: RectAxis, M: Material> {
    /// Dictates to which axles the rectangle is aligned to
    pub in_plane: A,
    pub a0: f64,
    pub a1: f64,
    pub b0: f64,
    pub b1: f64,
    /// Coord of the plane the rectangle is aligned to
    pub k: f64,
    pub material: M,
}

pub struct XY;
impl RectAxis for XY {
    const AXIS: Axis = Axis::Z;
    const OTHER1: Axis = Axis::X;
    const OTHER2: Axis = Axis::Y;
}

pub struct XZ;
impl RectAxis for XZ {
    const AXIS: Axis = Axis::Y;
    const OTHER1: Axis = Axis::X;
    const OTHER2: Axis = Axis::Z;
}

pub struct YZ;
impl RectAxis for YZ {
    const AXIS: Axis = Axis::X;
    const OTHER1: Axis = Axis::Y;
    const OTHER2: Axis = Axis::Z;
}

impl<A: RectAxis, M: Material> Hittable for Rect<A, M> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.origin[A::AXIS]) / r.direction[A::AXIS];

        if t < t_min || t > t_max {
            return None;
        }

        let a = r.origin[A::OTHER1] + t * r.direction[A::OTHER1];
        let b = r.origin[A::OTHER2] + t * r.direction[A::OTHER2];
        if a < self.a0 || a > self.a1 || b < self.b0 || b > self.b1 {
            return None;
        }

        let u = (a - self.a0) / (self.a1 - self.a0);
        let v = (b - self.b0) / (self.b1 - self.b0);
        let outward_normal = {
            let mut v = Vec3::zero();
            v[A::AXIS] = 1.0;
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
            &self.material,
        ))
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        // Added padding in 3rd dimension to avoid it being 0
        let mut v_min = Vec3::zero();
        v_min[A::OTHER1] = self.a0;
        v_min[A::OTHER2] = self.b0;
        v_min[A::AXIS] = self.k - AABB_ZERO_PADDING;
        let mut v_max = Vec3::zero();
        v_max[A::OTHER1] = self.a1;
        v_max[A::OTHER2] = self.b1;
        v_max[A::AXIS] = self.k + AABB_ZERO_PADDING;
        Some(AABB {
            minimum: v_min,
            maximum: v_max,
        })
    }
}
