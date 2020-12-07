use crate::hittable::{HittableList, Hittable};
use crate::material::*;
use crate::objects::Sphere;
use crate::vec3::Vec3;

use rand::Rng;

pub fn randon_spheres() -> HittableList {

    let mut objects: Vec<Box<dyn Hittable>> = Vec::new();

    // Add the ground
    objects.push(Box::new(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Lambertian {
            albedo: Vec3::new(0.5, 0.5, 0.5),
        },
    }));

    for a in -11..=11 {
        for b in -11..=11 {
            let choose_mat: f64 = rand::random();
            let center = Vec3::new(
                (a as f64) + 0.9 * rand::random::<f64>(),
                0.2,
                (b as f64) + 0.9 * rand::random::<f64>(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Vec3::random() * Vec3::random();

                    objects.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Lambertian { albedo },
                    }));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Vec3::random_in_range(0.5, 1.0);
                    let fuzz = rand::thread_rng().gen_range(0.0, 0.5);

                    objects.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Metal { albedo, fuzz },
                    }));
                } else {
                    // glass
                    objects.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Dielectric {
                            index_refraction: 1.5,
                        },
                    }));
                }
            }
        }
    }

    // Big glass boi
    objects.push(Box::new(Sphere {
        center: Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Dielectric {
            index_refraction: 1.5,
        },
    }));

    // Big diffuse boi
    objects.push(Box::new(Sphere {
        center: Vec3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Lambertian {
            albedo: Vec3::new(0.4, 0.2, 0.1),
        },
    }));

    // Big metal boi
    objects.push(Box::new(Sphere {
        center: Vec3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Metal {
            albedo: Vec3::new(0.7, 0.6, 0.5),
            fuzz: 0.0,
        },
    }));

    HittableList { objects }
}
