use super::*;

#[derive(Clone)]
pub struct CheckerTexture<T1: Texture, T2: Texture> {
    odd: T1,
    even: T2,
}

impl CheckerTexture<SolidColor, SolidColor> {
    pub fn from_colors(odd: Vec3, even: Vec3) -> Self {
        Self {
            odd: SolidColor::new(odd),
            even: SolidColor::new(even),
        }
    }
}

impl<T1: Texture, T2: Texture> Texture for CheckerTexture<T1, T2> {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        let p_10 = p.scale(10.0);
        let sines = p_10.x().sin() * p_10.y().sin() * p_10.z().sin();

        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
