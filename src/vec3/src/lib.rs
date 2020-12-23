use rand::Rng;
use std::cmp::PartialEq;
use std::convert::From;
use std::ops::Deref;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::ops::{Index, IndexMut};
use std::iter::FromIterator;

/// This sets the error while comparing floats in Vec3s.
const FLOAT_CMP_ERROR: f64 = 1e-8;

/// Axis of the Vec3. Can be used for indexing
pub enum Axis {
    X = 0,
    Y = 1,
    Z = 2,
}

/// Three dimensional vector with opertors.
///
/// It can be used for representing a point, a direction,
/// a RGB color or anything you may need it to.
#[derive(Debug, Clone, Copy, Default)]
pub struct Vec3 {
    pub v: [f64; 3],
}

impl Vec3 {
    /// Creates a new vector with the given coords.
    #[must_use]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { v: [x, y, z] }
    }

    /// Creates a new vector with all coords to `v`
    #[must_use]
    pub fn splat(v: f64) -> Self {
        Vec3 { v: [v; 3] }
    }

    /// Creates a new vector with all cords set to 0.
    #[must_use]
    pub const fn zero() -> Self {
        Vec3 { v: [0.0f64; 3] }
    }

    /// Creates a new vector with all coords set to 1.
    #[must_use]
    pub const fn one() -> Self {
        Vec3 { v: [1.0f64; 3] }
    }

    /// Creates a new random vector with coords in range \[0.0, 1.0\]
    #[must_use]
    #[inline]
    pub fn random() -> Self {
        Vec3 {
            v: [rand::random(), rand::random(), rand::random()],
        }
    }

    /// Creates a new random vector with coords in range [`min`, `max`]
    #[must_use]
    pub fn random_in_range(min: f64, max: f64) -> Self {
        let mut rng = rand::thread_rng();
        Vec3 {
            v: [
                rng.gen_range(min, max),
                rng.gen_range(min, max),
                rng.gen_range(min, max),
            ],
        }
    }

    #[must_use]
    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Self::random_in_range(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    #[must_use]
    #[inline]
    pub fn random_unit_vector() -> Self {
        Self::random_in_unit_sphere().unit_vector()
    }

    #[must_use]
    pub fn random_in_hemisphere(normal: &Vec3) -> Self {
        let in_unit_sphere = Self::random_in_unit_sphere();
        if in_unit_sphere.dot(normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    #[must_use]
    pub fn random_in_unit_disk() -> Self {
        let mut rng = rand::thread_rng();
        loop {
            let p = Self::new(rng.gen_range(-1.0, 1.0), rng.gen_range(-1.0, 1.0), 0.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn near_zero(&self) -> bool {
        (self.v[0].abs() < FLOAT_CMP_ERROR)
            && (self.v[1].abs() < FLOAT_CMP_ERROR)
            && (self.v[2].abs() < FLOAT_CMP_ERROR)
    }

    /// X coord getter
    #[inline]
    pub fn x(&self) -> f64 {
        self.v[0]
    }

    /// Y coord getter
    #[inline]
    pub fn y(&self) -> f64 {
        self.v[1]
    }

    /// Z coord getter
    #[inline]
    pub fn z(&self) -> f64 {
        self.v[2]
    }

    /// Calculates the length of the vector
    #[inline]
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    /// Calculates the squared lenght of the vetor.
    ///
    /// This is faster than only the [Vec3.length].
    #[inline]
    pub fn length_squared(&self) -> f64 {
        let v = &self.v;
        v[0] * v[0] + v[1] * v[1] + v[2] * v[2]
    }

    /// Scales the vector in place multiplying every coord by `factor`.
    pub fn scale_mut(&mut self, factor: f64) -> &mut Self {
        self.v[0] *= factor;
        self.v[1] *= factor;
        self.v[2] *= factor;
        self
    }

    /// Sacales the vector multiplying every coord by `factor`.
    #[must_use]
    pub fn scale(mut self, factor: f64) -> Self {
        self.scale_mut(factor);
        self
    }

    /// Calculates the dot product of two vectors.
    #[inline]
    pub fn dot(&self, other: &Self) -> f64 {
        self.v[0] * other.v[0] + self.v[1] * other.v[1] + self.v[2] * other.v[2]
    }

    /// Calculates the cross product of two vectors in place of the left side one.
    pub fn cross_mut(&mut self, other: &Self) -> &mut Self {
        self.v = [
            self.v[1] * other.v[2] - self.v[2] * other.v[1],
            self.v[2] * other.v[0] - self.v[0] * other.v[2],
            self.v[0] * other.v[1] - self.v[1] * other.v[0],
        ];
        self
    }

    /// Calculates the cross product of two vectors.
    #[must_use]
    pub fn cross(mut self, other: &Self) -> Self {
        self.cross_mut(other);
        self
    }

    /// Turns the vector into an unit vector in place.
    pub fn unit_vector_mut(&mut self) -> &mut Self {
        self.div_assign(self.length());
        self
    }

    /// Turns the vector into an unit vector.
    #[must_use]
    pub fn unit_vector(mut self) -> Self {
        self.unit_vector_mut();
        self
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<f64> {
        self.v.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> std::slice::IterMut<f64> {
        self.v.iter_mut()
    }

    #[inline]
    pub fn reduce(self, f: impl Fn(f64, f64) -> f64) -> f64 {
        f(f(self[0], self[1]), self[2])
    }

    #[must_use]
    #[inline]
    pub fn map(self, mut f: impl FnMut(f64) -> f64) -> Self {
        Vec3::new(f(self[0]), f(self[1]), f(self[2]))
    }

    #[must_use]
    #[inline]
    pub fn zip_with(self, other: Vec3, mut f: impl FnMut(f64, f64) -> f64) -> Self {
        Vec3::new(f(self[0], other[0]), f(self[1], other[1]), f(self[2], other[2]))
    }

    #[must_use]
    #[inline]
    pub fn zip_with3(
        self,
        other1: Vec3,
        other2: Vec3,
        mut f: impl FnMut(f64, f64, f64) -> f64,
    ) -> Self {
        Vec3::new(
            f(self[0], other1[0], other2[0]),
            f(self[1], other1[1], other2[1]),
            f(self[2], other1[2], other2[2]),
        )
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.v[0] += other.v[0];
        self.v[1] += other.v[1];
        self.v[2] += other.v[2];
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(mut self, other: Self) -> Self {
        self += other;
        self
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        self.v[0] -= other.v[0];
        self.v[1] -= other.v[1];
        self.v[2] -= other.v[2];
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(mut self, other: Self) -> Self {
        self -= other;
        self
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        let mut v = [0f64; 3];
        v[0] = -self.v[0];
        v[1] = -self.v[1];
        v[2] = -self.v[2];
        Self { v }
    }
}

impl MulAssign<f64> for Vec3 {
    /// Overloaded operator for [Vec3.scale_mut]
    fn mul_assign(&mut self, other: f64) {
        self.scale_mut(other);
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    /// Overloaded operator for [Vec3.scale]
    fn mul(mut self, other: f64) -> Self {
        self *= other;
        self
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    /// Overloaded operator for [Vec3.scale]
    fn mul(self, other: Vec3) -> Self::Output {
        other * self
    }
}

impl MulAssign<Vec3> for Vec3 {
    /// Multiplies elements one to one in place
    fn mul_assign(&mut self, other: Vec3) {
        self.v[0] *= other.v[0];
        self.v[1] *= other.v[1];
        self.v[2] *= other.v[2];
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    /// Multiplies elements one to one
    fn mul(mut self, other: Vec3) -> Self::Output {
        self *= other;
        self
    }
}

impl DivAssign<f64> for Vec3 {
    /// Overloaded operator for [Vec3.scale_mut] by `1.0 / factor`
    fn div_assign(&mut self, other: f64) {
        self.scale_mut(1.0f64 / other);
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    /// Overloaded operator for [Vec3.scale] by `1.0 / factor`
    fn div(mut self, other: f64) -> Self {
        self /= other;
        self
    }
}

impl Div<Vec3> for f64 {
    type Output = Vec3;

    /// Overloaded operator for [Vec3.scale] by `1.0 / factor`
    fn div(self, other: Vec3) -> Self::Output {
        other / self
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        (self.v[0] - other.v[0]).abs() < FLOAT_CMP_ERROR
            && (self.v[1] - other.v[1]).abs() < FLOAT_CMP_ERROR
            && (self.v[2] - other.v[2]).abs() < FLOAT_CMP_ERROR
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.v[index]
    }
}

impl Index<Axis> for Vec3 {
    type Output = f64;
    #[inline]
    fn index(&self, index: Axis) -> &Self::Output {
        &self.v[index as usize]
    }
}

impl IndexMut<usize> for Vec3 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.v[index]
    }
}

impl IndexMut<Axis> for Vec3 {
    #[inline]
    fn index_mut(&mut self, index: Axis) -> &mut Self::Output {
        &mut self.v[index as usize]
    }
}

impl Deref for Vec3 {
    type Target = [f64; 3];

    fn deref(&self) -> &Self::Target {
        &self.v
    }
}

impl From<[f64; 3]> for Vec3 {
    fn from(v: [f64; 3]) -> Self {
        Self { v }
    }
}

impl FromIterator<f64> for Vec3 {
    fn from_iter<T: IntoIterator<Item = f64>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        Self {
            v: [
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap(),
            ]
        }
    }
}

impl std::iter::Sum for Vec3 {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Vec3::default(), std::ops::Add::add)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_eq_float {
        ($lhs:expr, $rhs:expr) => {
            if ($lhs - $rhs).abs() >= FLOAT_CMP_ERROR {
                panic!();
            }
        };
    }

    macro_rules! assert_eq_vec3 {
        ($lhs:expr, $rhs:expr) => {
            assert_eq_float!($lhs.v[0], $rhs.v[0]);
            assert_eq_float!($lhs.v[1], $rhs.v[1]);
            assert_eq_float!($lhs.v[2], $rhs.v[2]);
        };
    }

    #[test]
    fn create() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v, Vec3 { v: [1.0, 2.0, 3.0] });
    }

    #[test]
    fn create_splat() {
        let v = Vec3::splat(14.0);
        assert_eq!(
            v,
            Vec3 {
                v: [14.0, 14.0, 14.0]
            }
        );
    }

    #[test]
    fn access() {
        let v = Vec3 { v: [1.0, 2.0, 3.0] };
        assert_eq!(v.x(), 1.0);
        assert_eq!(v.y(), 2.0);
        assert_eq!(v.z(), 3.0);
    }

    #[test]
    fn vector_length() {
        let v = Vec3 { v: [1.0, 2.0, 3.0] };
        assert_eq_float!(v.length_squared(), 14.0);
        assert_eq_float!(v.length_squared().sqrt(), 14f64.sqrt());
    }

    #[test]
    fn scale_vector() {
        let v = Vec3 { v: [1.0, 2.0, 3.0] };
        let r = Vec3 { v: [2.0, 4.0, 6.0] };
        let v_test = v.scale(2.0);
        assert_eq_vec3!(v_test, r);
        let mut v = Vec3 { v: [1.0, 2.0, 3.0] };
        v.scale_mut(2.0);
        assert_eq_vec3!(v, r);
    }

    #[test]
    fn dot_product() {
        let v1 = Vec3 { v: [1.0, 2.0, 3.0] };
        let v2 = Vec3 { v: [3.0, 2.0, 1.0] };
        assert_eq_float!(v1.dot(&v2), 10.0);
    }

    #[test]
    fn cross_product() {
        let v1 = Vec3 {
            v: [4.0, -2.0, 1.0],
        };
        let v2 = Vec3 {
            v: [1.0, -1.0, 3.0],
        };
        let r = Vec3 {
            v: [-5.0, -11.0, -2.0],
        };
        let v1_test = v1.cross(&v2);
        assert_eq_vec3!(v1_test, r);
        let mut v1 = Vec3 {
            v: [4.0, -2.0, 1.0],
        };
        v1.cross_mut(&v2);
        assert_eq!(v1, r);
    }

    #[test]
    fn to_unit_vector() {
        let v = Vec3 { v: [1.0, 2.0, 3.0] };
        assert_eq_float!(v.unit_vector().length_squared(), 1.0);
    }

    #[test]
    fn add_vectors() {
        let v1 = Vec3 { v: [1.0, 2.0, 3.0] };
        let v2 = Vec3 { v: [3.0, 2.0, 1.0] };
        let r = Vec3 { v: [4.0, 4.0, 4.0] };
        let r_test = v1 + v2;
        assert_eq_vec3!(r_test, r);
        let mut v1 = Vec3 { v: [1.0, 2.0, 3.0] };
        let v2 = Vec3 { v: [3.0, 2.0, 1.0] };
        v1 += v2;
        assert_eq_vec3!(v1, r);
    }

    #[test]
    fn subtract_vectors() {
        let v1 = Vec3 { v: [1.0, 2.0, 3.0] };
        let v2 = Vec3 { v: [3.0, 2.0, 1.0] };
        let r = Vec3 {
            v: [-2.0, 0.0, 2.0],
        };
        let r_test = v1 - v2;
        assert_eq_vec3!(r_test, r);
        let mut v1 = Vec3 { v: [1.0, 2.0, 3.0] };
        let v2 = Vec3 { v: [3.0, 2.0, 1.0] };
        v1 -= v2;
        assert_eq_vec3!(v1, r);
    }

    #[test]
    fn negate_vector() {
        let v = Vec3 { v: [1.0, 2.0, 3.0] };
        let r = Vec3 {
            v: [-1.0, -2.0, -3.0],
        };
        let vr = -v;
        assert_eq_vec3!(vr, r);
    }

    #[test]
    fn vector_scale_operators() {
        let original = Vec3 { v: [1.0, 2.0, 3.0] };
        let r = Vec3 { v: [2.0, 4.0, 6.0] };
        let v1 = original.clone() * 2.0;
        assert_eq_vec3!(v1, r);
        let v2 = v1 / 2.0;
        assert_eq_vec3!(v2, original);
        let v3 = 2.0 * v2;
        assert_eq_vec3!(v3, r);
    }

    #[test]
    fn vector_partial_eq() {
        let v1 = Vec3 { v: [1.0, 2.0, 3.0] };
        let v2 = Vec3 { v: [1.1, 2.0, 3.0] };
        assert!(v1 == v1);
        assert!(v1 != v2);
        assert!(v1 != v1 + Vec3 { v: [1.0; 3] } * FLOAT_CMP_ERROR * 10.0);
        assert!(v1 == v1 + Vec3 { v: [1.0; 3] } * (FLOAT_CMP_ERROR / 10.0));
    }

    #[test]
    fn index_operator() {
        let mut v = Vec3 { v: [1.0, 2.0, 3.0] };
        let v_clone = v.clone();
        assert_eq!(v.x(), v[0]);
        assert_eq!(v.y(), v[1]);
        assert_eq!(v.z(), v[2]);
        v[0] += 1.0;
        v[1] += 1.0;
        v[2] += 1.0;
        assert_eq_vec3!(v, v_clone + Vec3 { v: [1.0; 3] });
    }
}
