use super::*;

/// Lambertian material
///
/// I think about this one like a plain color/texture
#[derive(Clone)]
pub struct Lambertian<T: Texture> {
    /// Texture the material scatters on hit
    pub albedo: T,
}

impl Lambertian<SolidColor> {
    /// Constructs a [Lambertian] material with a [SolidColor] as a texture
    pub fn from_color(color: Vec3) -> Self {
        Self {
            albedo: SolidColor { color },
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
