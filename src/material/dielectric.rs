use super::*;

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
