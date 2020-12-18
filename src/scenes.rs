use crate::camera::CameraConfig;
use crate::hittable::{Hittable, HittableList};
use crate::material::*;
use crate::objects::*;
use crate::texture::*;
use crate::Config;
use std::sync::Arc;
use vec3::Vec3;

use rand::Rng;

pub fn gen_scene_from_name(c: &Config) -> Option<(HittableList, CameraConfig)> {
    match c.scene_name.as_str() {
        "spheres" => Some((random_spheres(), default_cam(c.aspect_ratio))),
        "bouncing_spheres" => Some((random_bouncing_spheres(), default_cam(c.aspect_ratio))),
        "checker_ground" => Some((random_spheres_checker(), default_cam(c.aspect_ratio))),
        "checker_spheres" => Some((checker_spheres(), aperture_0(c.aspect_ratio))),
        "perlin_spheres" => Some((perlin_spheres(), aperture_0(c.aspect_ratio))),
        _ => None,
    }
}

fn default_cam(aspect_ratio: f64) -> CameraConfig {
    CameraConfig {
        lookfrom: Vec3::new(12.0, 2.0, 3.0),
        lookat: Vec3::new(0.0, 0.0, 0.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
        focus_distance: 10.0,
        vfov: 20.0,
        aperture: 0.1,
        time0: 0.0,
        time1: 1.0,
        aspect_ratio,
    }
}

pub fn random_spheres() -> HittableList {
    let mut objects: Vec<Arc<dyn Hittable>> = Vec::new();

    // Add the ground
    objects.push(Arc::new(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Arc::new(Lambertian::from_color(Vec3::new(0.5, 0.5, 0.5))),
    }));

    let glass = Arc::new(Dielectric {
        index_refraction: 1.5,
    });

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
                    let color = Vec3::random() * Vec3::random();

                    objects.push(Arc::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Arc::new(Lambertian::from_color(color)),
                    }));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Vec3::random_in_range(0.5, 1.0);
                    let fuzz = rand::thread_rng().gen_range(0.0, 0.5);

                    objects.push(Arc::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Arc::new(Metal { albedo, fuzz }),
                    }));
                } else {
                    // glass
                    objects.push(Arc::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Arc::clone(&glass),
                    }));
                }
            }
        }
    }

    // Big glass boi
    objects.push(Arc::new(Sphere {
        center: Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::clone(&glass),
    }));

    // Big diffuse boi
    objects.push(Arc::new(Sphere {
        center: Vec3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Lambertian::from_color(Vec3::new(0.4, 0.2, 0.1))),
    }));

    // Big metal boi
    objects.push(Arc::new(Sphere {
        center: Vec3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Metal {
            albedo: Vec3::new(0.7, 0.6, 0.5),
            fuzz: 0.0,
        }),
    }));

    HittableList { objects }
}

pub fn random_bouncing_spheres() -> HittableList {
    let mut objects: Vec<Arc<dyn Hittable>> = Vec::new();

    // Add the ground
    objects.push(Arc::new(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Arc::new(Lambertian::from_color(Vec3::new(0.5, 0.5, 0.5))),
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
                    let color = Vec3::random() * Vec3::random();

                    let center1 = center + Vec3::new(0.0, rand::random::<f64>() * 0.5, 0.0);

                    objects.push(Arc::new(MovingSphere {
                        center0: center,
                        center1,
                        time0: 0.0,
                        time1: 1.0,
                        radius: 0.2,
                        material: Lambertian::from_color(color),
                    }));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Vec3::random_in_range(0.5, 1.0);
                    let fuzz = rand::thread_rng().gen_range(0.0, 0.5);

                    objects.push(Arc::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Arc::new(Metal { albedo, fuzz }),
                    }));
                } else {
                    // glass
                    objects.push(Arc::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Arc::new(Dielectric {
                            index_refraction: 1.5,
                        }),
                    }));
                }
            }
        }
    }

    // Big glass boi
    objects.push(Arc::new(Sphere {
        center: Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Dielectric {
            index_refraction: 1.5,
        }),
    }));

    // Big diffuse boi
    objects.push(Arc::new(Sphere {
        center: Vec3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Lambertian::from_color(Vec3::new(0.4, 0.2, 0.1))),
    }));

    // Big metal boi
    objects.push(Arc::new(Sphere {
        center: Vec3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Metal {
            albedo: Vec3::new(0.7, 0.6, 0.5),
            fuzz: 0.0,
        }),
    }));

    HittableList { objects }
}

pub fn random_spheres_checker() -> HittableList {
    let mut objects: Vec<Arc<dyn Hittable>> = Vec::new();

    // Add the ground
    let cheker = CheckerTexture::from_colors(Vec3::new(0.2, 0.3, 0.1), Vec3::new(0.9, 0.9, 0.9));
    objects.push(Arc::new(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Arc::new(Lambertian {
            albedo: Arc::new(cheker),
        }),
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
                    let color = Vec3::random() * Vec3::random();

                    objects.push(Arc::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Arc::new(Lambertian::from_color(color)),
                    }));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Vec3::random_in_range(0.5, 1.0);
                    let fuzz = rand::thread_rng().gen_range(0.0, 0.5);

                    objects.push(Arc::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Arc::new(Metal { albedo, fuzz }),
                    }));
                } else {
                    // glass
                    objects.push(Arc::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Arc::new(Dielectric {
                            index_refraction: 1.5,
                        }),
                    }));
                }
            }
        }
    }

    // Big glass boi
    objects.push(Arc::new(Sphere {
        center: Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Dielectric {
            index_refraction: 1.5,
        }),
    }));

    // Big diffuse boi
    objects.push(Arc::new(Sphere {
        center: Vec3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Lambertian::from_color(Vec3::new(0.4, 0.2, 0.1))),
    }));

    // Big metal boi
    objects.push(Arc::new(Sphere {
        center: Vec3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Metal {
            albedo: Vec3::new(0.7, 0.6, 0.5),
            fuzz: 0.0,
        }),
    }));

    HittableList { objects }
}

pub fn aperture_0(aspect_ratio: f64) -> CameraConfig {
    let default = default_cam(aspect_ratio);
    CameraConfig {
        aperture: 0.0,
        ..default
    }
}

pub fn checker_spheres() -> HittableList {
    let mut objects: Vec<Arc<dyn Hittable>> = Vec::new();

    let checker: Arc<dyn Texture> = Arc::new(CheckerTexture::from_colors(
        Vec3::new(0.2, 0.3, 0.1),
        Vec3::new(0.9, 0.9, 0.9),
    ));
    let material = Arc::new(Lambertian {
        albedo: Arc::clone(&checker),
    });
    objects.push(Arc::new(Sphere {
        center: Vec3::new(0.0, -10.0, 0.0),
        radius: 10.0,
        material: Arc::clone(&material),
    }));
    objects.push(Arc::new(Sphere {
        center: Vec3::new(0.0, 10.0, 0.0),
        radius: 10.0,
        material: Arc::clone(&material),
    }));

    HittableList { objects }
}

pub fn perlin_spheres() -> HittableList {
    let mut objects: Vec<Arc<dyn Hittable>> = Vec::new();

    let texture = Arc::new(NoiseTexture::new(4.0));
    let material = Arc::new(Lambertian { albedo: texture });

    objects.push(Arc::new(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Arc::clone(&material),
    }));

    objects.push(Arc::new(Sphere {
        center: Vec3::new(0.0, 2.0, 0.0),
        radius: 2.0,
        material: Arc::clone(&material),
    }));

    HittableList { objects }
}
