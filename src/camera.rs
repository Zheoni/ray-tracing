use crate::ray::Ray;
use vec3::Vec3;

use rand::Rng;

pub struct CameraConfig {
    pub lookfrom: Vec3,
    pub lookat: Vec3,
    pub vup: Vec3,
    pub vfov: f64,
    pub aperture: f64,
    pub focus_distance: f64,
    pub time0: f64,
    pub time1: f64,
}

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    #[allow(dead_code)]
    w: Vec3,
    lens_radius: f64,
    time0: f64,
    time1: f64,
}

impl Camera {
    pub fn new(c: &CameraConfig, aspect_ratio: f64) -> Self {
        let theta = c.vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (c.lookfrom - c.lookat).unit_vector();
        let u = c.vup.cross(&w).unit_vector();
        let v = w.cross(&u);

        let origin = c.lookfrom;
        let horizontal = c.focus_distance * viewport_width * u;
        let vertical = c.focus_distance * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - c.focus_distance * w;

        let lens_radius = c.aperture / 2.0;
        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            w,
            u,
            v,
            lens_radius,
            time0: c.time0,
            time1: c.time1,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset,
            rand::thread_rng().gen_range(self.time0..self.time1),
        )
    }
}
