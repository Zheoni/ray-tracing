use vec3::Vec3;

const POINT_COUNT: usize = 256;

#[derive(Clone)]
pub struct PerlinNoiseGenerator {
    ranvec: [Vec3; POINT_COUNT],
    perm_x: [usize; POINT_COUNT],
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT],
}

pub type PNG = PerlinNoiseGenerator;

impl Default for PerlinNoiseGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl PerlinNoiseGenerator {
    pub fn noise(&self, p: &[f64; 3]) -> f64 {
        let u = p[0] - p[0].floor();
        let v = p[1] - p[1].floor();
        let w = p[2] - p[2].floor();

        let i = p[0].floor() as isize;
        let j = p[1].floor() as isize;
        let k = p[2].floor() as isize;
        let mut c = [[[Vec3::zero(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ranvec[self.perm_x
                        [((i + di as isize) & (POINT_COUNT - 1) as isize) as usize]
                        ^ self.perm_y[((j + dj as isize) & (POINT_COUNT - 1) as isize) as usize]
                        ^ self.perm_z[((k + dk as isize) & (POINT_COUNT - 1) as isize) as usize]]
                }
            }
        }
        Self::trilinear_interpolation(&c, u, v, w)
    }

    pub fn turbulence(&self, p: &Vec3) -> f64 {
        self.turbulence_with_depth(p, 7)
    }

    pub fn turbulence_with_depth(&self, p: &Vec3, depth: usize) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        accum.abs()
    }
}

impl PerlinNoiseGenerator {
    pub fn new() -> Self {
        let mut ranvec = [Vec3::zero(); POINT_COUNT];
        for v in ranvec.iter_mut() {
            *v = Vec3::random_in_range(-1.0, 1.0).unit_vector();
        }
        let perm_x = Self::generate_perm();
        let perm_y = Self::generate_perm();
        let perm_z = Self::generate_perm();
        Self {
            ranvec,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    fn generate_perm() -> [usize; POINT_COUNT] {
        let mut perm = [0; POINT_COUNT];
        for (i, p) in perm.iter_mut().enumerate() {
            *p = i;
        }

        Self::permute(&mut perm);

        perm
    }

    fn permute(perm: &mut [usize; POINT_COUNT]) {
        for i in (0..perm.len()).rev() {
            use rand::Rng;
            let target = rand::thread_rng().gen_range(0, i + 1);
            perm.swap(i, target);
        }
    }

    fn trilinear_interpolation(c: &[[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);

        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let (fi, fj, fk) = (i as f64, j as f64, k as f64);
                    let weight_v = Vec3::new(u - fi, v - fj, w - fk);
                    accum += (fi * uu + (1.0 - fi) * (1.0 - uu))
                        * (fj * vv + (1.0 - fj) * (1.0 - vv))
                        * (fk * ww + (1.0 - fk) * (1.0 - ww))
                        * c[i][j][k].dot(&weight_v);
                }
            }
        }
        accum
    }
}
