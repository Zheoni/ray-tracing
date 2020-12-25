use super::*;

pub struct Block {
    pub p0: Vec3,
    pub p1: Vec3,
    sides: HittableList,
}

impl Block {
    pub fn new<M: 'static + Material + Clone>(p0: Vec3, p1: Vec3, material: M) -> Self {
        let mut side_rectangles: Vec<Box<dyn Hittable>> = Vec::new();

        side_rectangles.push(Box::new(Rect {
            in_plane: XY,
            a0: p0.x(),
            a1: p1.x(),
            b0: p0.y(),
            b1: p1.y(),
            k: p1.z(),
            material: material.clone(),
        }));
        side_rectangles.push(Box::new(Rect {
            in_plane: XY,
            a0: p0.x(),
            a1: p1.x(),
            b0: p0.y(),
            b1: p1.y(),
            k: p0.z(),
            material: material.clone(),
        }));

        side_rectangles.push(Box::new(Rect {
            in_plane: XZ,
            a0: p0.x(),
            a1: p1.x(),
            b0: p0.z(),
            b1: p1.z(),
            k: p1.y(),
            material: material.clone(),
        }));
        side_rectangles.push(Box::new(Rect {
            in_plane: XZ,
            a0: p0.x(),
            a1: p1.x(),
            b0: p0.z(),
            b1: p1.z(),
            k: p0.y(),
            material: material.clone(),
        }));

        side_rectangles.push(Box::new(Rect {
            in_plane: YZ,
            a0: p0.y(),
            a1: p1.y(),
            b0: p0.z(),
            b1: p1.z(),
            k: p1.x(),
            material: material.clone(),
        }));
        side_rectangles.push(Box::new(Rect {
            in_plane: YZ,
            a0: p0.y(),
            a1: p1.y(),
            b0: p0.z(),
            b1: p1.z(),
            k: p0.x(),
            material,
        }));

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
