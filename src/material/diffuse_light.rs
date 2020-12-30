use super::*;

/// Diffuse light material
///
/// Does not scatters lay but emits a texture
#[derive(Clone)]
pub struct DiffuseLight<T: Texture> {
    /// Texture the material emits
    emit: T,
}

impl DiffuseLight<SolidColor> {
    /// Constructs a [DiffuseLight] material with a [SolidColor] as a texture
    pub fn from_color(color: Vec3) -> Self {
        Self {
            emit: SolidColor { color },
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
