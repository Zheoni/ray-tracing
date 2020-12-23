use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::texture::{SolidColor, Texture};
use std::ops::Neg;
use vec3::Vec3;

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, hit: &HitRecord) -> Option<(Vec3, Ray)>;
    fn emitted(&self, _u: f64, _v: f64, _p: &Vec3) -> Vec3 {
        Vec3::zero()
    }
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    (*v) - 2.0 * (v.dot(n)) * (*n)
}

fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = uv.neg().dot(n).min(1.0);
    let r_out_perp = etai_over_etat * ((*uv) + cos_theta * (*n));
    let r_out_parallel = (1.0 - r_out_perp.length_squared()).abs().sqrt().neg() * (*n);
    r_out_perp + r_out_parallel
}

#[derive(Clone)]
pub struct Lambertian<T: Texture> {
    pub albedo: T,
}

impl Lambertian<SolidColor> {
    pub fn from_color(color: Vec3) -> Self {
        Self {
            albedo: SolidColor::new(color),
        }
    }
}

impl<T: Texture> Material for Lambertian<T> {
    fn scatter(&self, r_in: &Ray, hit: &HitRecord) -> Option<(Vec3, Ray)> {
        let mut scatter_direction = hit.normal + Vec3::random_unit_vector();

        // Catch scatter direction near 0
        if scatter_direction.near_zero() {
            scatter_direction = hit.normal;
        }

        let scattered = Ray::new(hit.point, scatter_direction, r_in.time);
        let attenuation = self.albedo.value(hit.u, hit.v, &hit.point);
        Some((attenuation, scattered))
    }
}

#[derive(Clone)]
pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hit: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = reflect(&r_in.direction.unit_vector(), &hit.normal);

        let fuzz = self.fuzz.min(1.0);

        let scattered = Ray::new(
            hit.point,
            reflected + fuzz * Vec3::random_in_unit_sphere(),
            r_in.time,
        );
        let attenuation = self.albedo;
        if scattered.direction.dot(&hit.normal) > 0.0 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}

#[derive(Clone)]
pub struct Dielectric {
    pub index_refraction: f64,
}

impl Dielectric {
    fn reflectance(&self, cosine: f64, ref_idx: f64) -> f64 {
        // Schlick's approximation
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, hit: &HitRecord) -> Option<(Vec3, Ray)> {
        let attenuation = Vec3::one();
        let refraction_ratio = if hit.front_face {
            1.0 / self.index_refraction
        } else {
            self.index_refraction
        };

        let unit_direction = r_in.direction.unit_vector();
        let cos_theta = unit_direction.neg().dot(&hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        // Cannot refract, no solution for Snell equation => reflect
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        // Some rays will be reflected
        let reflectance = self.reflectance(cos_theta, refraction_ratio);

        let direction = if cannot_refract || reflectance > rand::random() {
            reflect(&unit_direction, &hit.normal)
        } else {
            refract(&unit_direction, &hit.normal, refraction_ratio)
        };

        let scattered = Ray::new(hit.point, direction, r_in.time);
        Some((attenuation, scattered))
    }
}

#[derive(Clone)]
pub struct DiffuseLight<T: Texture> {
    emit: T,
}

impl DiffuseLight<SolidColor> {
    pub fn from_color(color: Vec3) -> Self {
        Self {
            emit: SolidColor::new(color),
        }
    }
}

impl<T: Texture> Material for DiffuseLight<T> {
    fn scatter(&self, _r_in: &Ray, _hit: &HitRecord) -> Option<(Vec3, Ray)> {
        None
    }

    fn emitted(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        self.emit.value(u, v, p)
    }
}

#[derive(Clone)]
pub struct Isotropic<T: Texture> {
    pub albedo: T,
}

impl Isotropic<SolidColor> {
    pub fn from_color(color: Vec3) -> Self {
        Self {
            albedo: SolidColor::new(color),
        }
    }
}

impl<T: Texture> Material for Isotropic<T> {
    fn scatter(&self, r_in: &Ray, hit: &HitRecord) -> Option<(Vec3, Ray)> {
        let attenuation = self.albedo.value(hit.u, hit.v, &hit.point);
        let scattered = Ray::new(hit.point, Vec3::random_in_unit_sphere(), r_in.time);
        Some((attenuation, scattered))
    }
}
