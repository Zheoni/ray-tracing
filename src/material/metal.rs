use super::*;

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
