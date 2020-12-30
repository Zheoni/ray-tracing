use super::*;

/// Isotropic material
///
/// Currently used as a smoke approximation as it scatters light
/// intoa random direction
#[derive(Clone)]
pub struct Isotropic<T: Texture> {
    /// Texture the material scatters
    pub albedo: T,
}

impl Isotropic<SolidColor> {
    /// Constructs a [Isotropic] material with a [SolidColor] as a texture
    pub fn from_color(color: Vec3) -> Self {
        Self {
            albedo: SolidColor { color },
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
