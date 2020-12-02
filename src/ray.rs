use crate::vec3::Vec3;

use crate::hittable::Hittable;

/// Representation of a Ray for the raytracer.
#[derive(Debug, Clone, PartialEq)]
pub struct Ray {
    /// Origin point of the Ray
    pub origin: Vec3,
    /// Direction of the Ray
    pub direction: Vec3,
}

impl Ray {
    /// Returns a Ray with the given origin and direction.
    ///
    /// # Arguments
    ///
    /// * `origin` - A Vec3 representing the point where the Ray started.
    /// * `direction` - A Vec3 representing the Ray direction. Usually a
    ///     unit vector but it is not enforced.
    ///
    /// Keep in mind that the Ray takes the ownership of the Vec3s when it is created.
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    /// Returns the position of the Ray *at* time `t`.
    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }

    pub fn ray_color(&self, world: &impl Hittable, depth: u32) -> Vec3 {
        // If maximum number of rays
        if depth == 0 {
            return Vec3::zero();
        }
    
        // If hit with some object. The min hit distance is not 0 because
        // of course float precission. Not every ray will match exactly with 0.0
        if let Some(hit) = world.hit(self, 0.001, f64::INFINITY) {
            if let Some((attenuation, scattered)) = hit.material.scatter(self, &hit) {
                return attenuation * scattered.ray_color(world, depth - 1);
            }
            return Vec3::zero();
        }
    
        // Background
        let unit_direction = self.direction.unit_vector();
        let t = (unit_direction.y() + 1.0) * 0.5;
        (1.0 - t) * Vec3::one() + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let origin = Vec3::new(1.0, 1.0, 1.0);
        let direction = Vec3::new(1.0, 0.0, 0.0);
        let ray = Ray::new(origin, direction);
        assert_eq!(ray, Ray { origin, direction });
    }

    #[test]
    fn ray_at() {
        let origin = Vec3::new(1.0, 1.0, 1.0);
        let direction = Vec3::new(1.0, 0.0, 0.0);
        let ray = Ray { origin, direction };
        assert_eq!(ray.at(14.0), Vec3::new(15.0, 1.0, 1.0));
    }
}