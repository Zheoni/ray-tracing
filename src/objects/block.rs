use super::*;

pub struct Block {
    p0: Vec3,
    p1: Vec3,
    sides: HittableList,
}

impl Block {
    pub fn new(p0: Vec3, p1: Vec3, material: Arc<dyn Material>) -> Self {
        let mut side_rectangles: Vec<Arc<dyn Hittable>> = Vec::new();

        side_rectangles.push(Arc::new(Rect::new(
            RectAxis::XY,
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p1.z(),
            Arc::clone(&material),
        )));
        side_rectangles.push(Arc::new(Rect::new(
            RectAxis::XY,
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p0.z(),
            Arc::clone(&material),
        )));

        side_rectangles.push(Arc::new(Rect::new(
            RectAxis::XZ,
            p0.x(),
            p1.x(),
            p0.z(),
            p1.z(),
            p1.y(),
            Arc::clone(&material),
        )));
        side_rectangles.push(Arc::new(Rect::new(
            RectAxis::XZ,
            p0.x(),
            p1.x(),
            p0.z(),
            p1.z(),
            p0.y(),
            Arc::clone(&material),
        )));

        side_rectangles.push(Arc::new(Rect::new(
            RectAxis::YZ,
            p0.y(),
            p1.y(),
            p0.z(),
            p1.z(),
            p1.x(),
            Arc::clone(&material),
        )));
        side_rectangles.push(Arc::new(Rect::new(
            RectAxis::YZ,
            p0.y(),
            p1.y(),
            p0.z(),
            p1.z(),
            p0.x(),
            Arc::clone(&material),
        )));

        Self {
            p0,
            p1,
            sides: HittableList {
                objects: side_rectangles,
            },
        }
    }
}

impl Hittable for Block {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(r, t_min, t_max)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        Some(AABB {
            minimum: self.p0,
            maximum: self.p1,
        })
    }
}
