use crate::camera::CameraConfig;
use crate::hittable::{Hittable, HittableList, RotateY, Translate};
use crate::material::*;
use crate::objects::*;
use crate::texture::*;
use crate::Config;
use std::sync::Arc;
use vec3::Vec3;

use rand::Rng;

pub struct Scene {
    pub world: HittableList,
    pub camera_config: CameraConfig,
    pub background_color: Vec3,
}

pub fn gen_scene_from_name(c: &Config) -> Option<Scene> {
    let def_cam = default_cam(c.aspect_ratio);
    let def_background = Vec3::new(0.7, 0.8, 1.0);

    match c.scene_name.as_str() {
        "spheres" => Some(Scene {
            world: random_spheres(),
            camera_config: def_cam,
            background_color: def_background,
        }),
        "bouncing_spheres" => Some(Scene {
            world: random_bouncing_spheres(),
            camera_config: def_cam,
            background_color: def_background,
        }),
        "checker_ground" => Some(Scene {
            world: random_spheres_checker(),
            camera_config: def_cam,
            background_color: def_background,
        }),
        "checker_spheres" => Some(Scene {
            world: checker_spheres(),
            camera_config: aperture_0(&def_cam),
            background_color: def_background,
        }),
        "perlin_spheres" => Some(Scene {
            world: perlin_spheres(),
            camera_config: aperture_0(&def_cam),
            background_color: def_background,
        }),
        "earth" => Some(Scene {
            world: earth(),
            camera_config: aperture_0(&def_cam),
            background_color: def_background,
        }),
        "black" => Some(Scene {
            world: HittableList {
                objects: Vec::new(),
            },
            camera_config: def_cam,
            background_color: Vec3::zero(),
        }),
        "simple_light" => Some(Scene {
            world: simple_light(),
            camera_config: simple_light_camera(&def_cam),
            background_color: Vec3::zero(),
        }),
        "cornell_box" => Some(Scene {
            world: cornell_box(),
            camera_config: cornell_box_camera(&def_cam),
            background_color: Vec3::zero(),
        }),
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

    let glass: Arc<dyn Material> = Arc::new(Dielectric {
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

pub fn aperture_0(default: &CameraConfig) -> CameraConfig {
    CameraConfig {
        aperture: 0.0,
        ..*default
    }
}

pub fn checker_spheres() -> HittableList {
    let mut objects: Vec<Arc<dyn Hittable>> = Vec::new();

    let checker: Arc<dyn Texture> = Arc::new(CheckerTexture::from_colors(
        Vec3::new(0.2, 0.3, 0.1),
        Vec3::new(0.9, 0.9, 0.9),
    ));
    let material: Arc<dyn Material> = Arc::new(Lambertian {
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
    let material: Arc<dyn Material> = Arc::new(Lambertian { albedo: texture });

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

pub fn earth() -> HittableList {
    let earth_texture = Arc::new(ImageTexture::new("earthmap.jpg").unwrap());
    let earth_surface = Arc::new(Lambertian {
        albedo: earth_texture,
    });
    let globe = Arc::new(Sphere {
        center: Vec3::zero(),
        radius: 2.0,
        material: earth_surface,
    });

    HittableList {
        objects: vec![globe],
    }
}

pub fn simple_light_camera(default: &CameraConfig) -> CameraConfig {
    CameraConfig {
        lookfrom: Vec3::new(26.0, 3.0, 6.0),
        lookat: Vec3::new(0.0, 2.0, 0.0),
        ..*default
    }
}

pub fn simple_light() -> HittableList {
    let mut objects: Vec<Arc<dyn Hittable>> = Vec::new();

    let marble: Arc<dyn Material> = Arc::new(Lambertian {
        albedo: Arc::new(NoiseTexture::new(4.0)),
    });

    objects.push(Arc::new(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Arc::clone(&marble),
    }));
    objects.push(Arc::new(Sphere {
        center: Vec3::new(0.0, 2.0, 0.0),
        radius: 2.0,
        material: Arc::clone(&marble),
    }));

    let difflight: Arc<dyn Material> = Arc::new(DiffuseLight::from_color(Vec3::splat(4.0)));
    objects.push(Arc::new(Rect::new(
        RectAxis::XY,
        3.0,
        5.0,
        1.0,
        3.0,
        -2.0,
        Arc::clone(&difflight),
    )));

    HittableList { objects }
}

pub fn cornell_box() -> HittableList {
    let mut objects: Vec<Arc<dyn Hittable>> = Vec::new();

    let red: Arc<dyn Material> = Arc::new(Lambertian::from_color(Vec3::new(0.65, 0.05, 0.05)));
    let white: Arc<dyn Material> = Arc::new(Lambertian::from_color(Vec3::splat(0.73)));
    let green: Arc<dyn Material> = Arc::new(Lambertian::from_color(Vec3::new(0.12, 0.45, 0.15)));
    let light: Arc<dyn Material> = Arc::new(DiffuseLight::from_color(Vec3::splat(15.0)));

    // Sides
    objects.push(Arc::new(Rect::new(
        RectAxis::YZ,
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        green,
    )));
    objects.push(Arc::new(Rect::new(
        RectAxis::YZ,
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        red,
    )));
    objects.push(Arc::new(Rect::new(
        RectAxis::XZ,
        213.0,
        343.0,
        227.0,
        332.0,
        554.0,
        light,
    )));
    objects.push(Arc::new(Rect::new(
        RectAxis::XZ,
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        Arc::clone(&white),
    )));
    objects.push(Arc::new(Rect::new(
        RectAxis::XZ,
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Arc::clone(&white),
    )));
    objects.push(Arc::new(Rect::new(
        RectAxis::XY,
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Arc::clone(&white),
    )));

    // Blocks
    let box1 = Arc::new(Block::new(
        Vec3::zero(),
        Vec3::new(165.0, 330.0, 165.0),
        Arc::clone(&white),
    ));
    let box1 = Arc::new(RotateY::new(box1, 15.0));
    let box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    objects.push(box1);

    let box2 = Arc::new(Block::new(
        Vec3::zero(),
        Vec3::new(165.0, 165.0, 165.0),
        Arc::clone(&white),
    ));
    let box2 = Arc::new(RotateY::new(box2, -18.0));
    let box2 = Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));
    objects.push(box2);

    HittableList { objects }
}

fn cornell_box_camera(default: &CameraConfig) -> CameraConfig {
    CameraConfig {
        lookfrom: Vec3::new(278.0, 278.0, -800.0),
        lookat: Vec3::new(278.0, 270.0, 0.0),
        vfov: 40.0,
        aperture: 0.0,
        ..*default
    }
}
