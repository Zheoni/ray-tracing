use crate::camera::CameraConfig;
use crate::constant_medium::ConstantMedium;
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
        "cornell_smoke" => Some(Scene {
            world: cornell_smoke(),
            camera_config: cornell_box_camera(&def_cam),
            background_color: Vec3::zero(),
        }),
        "final_scene" => Some(Scene {
            world: final_scene(),
            camera_config: final_scene_camera(&def_cam),
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

pub fn cornell_smoke() -> HittableList {
    let mut cb = cornell_box();
    let box1 = cb.objects.pop().unwrap();
    let box2 = cb.objects.pop().unwrap();

    cb.objects.push(Arc::new(ConstantMedium::from_color(
        box1,
        0.01,
        Vec3::zero(),
    )));
    cb.objects.push(Arc::new(ConstantMedium::from_color(
        box2,
        0.01,
        Vec3::one(),
    )));

    cb
}

pub fn final_scene() -> HittableList {
    use crate::bvh::BVHNode;
    let mut rng = rand::thread_rng();
    let mut boxes: Vec<Arc<dyn Hittable>> = Vec::new();

    let ground: Arc<dyn Material> = Arc::new(Lambertian::from_color(Vec3::new(0.48, 0.83, 0.53)));

    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = rng.gen_range(1.0, 101.0);
            let z1 = z0 + w;

            boxes.push(Arc::new(Block::new(
                Vec3::new(x0, y0, z0),
                Vec3::new(x1, y1, z1),
                Arc::clone(&ground),
            )));
        }
    }

    let mut objects: Vec<Arc<dyn Hittable>> = Vec::new();

    objects.push(Arc::new(BVHNode::build_tree(&mut boxes, 0.0, 1.0)));

    let light: Arc<dyn Material> = Arc::new(DiffuseLight::from_color(Vec3::splat(7.0)));
    objects.push(Arc::new(Rect::new(
        RectAxis::XZ,
        123.0,
        423.0,
        147.0,
        412.0,
        445.0,
        Arc::clone(&light),
    )));

    let center0 = Vec3::new(400.0, 400.0, 200.0);
    let center1 = center0 + Vec3::new(30.0, 0.0, 0.0);
    let moving_sphere_material = Arc::new(Lambertian::from_color(Vec3::new(0.7, 0.3, 0.1)));
    objects.push(Arc::new(MovingSphere {
        center0,
        center1,
        time0: 0.0,
        time1: 1.0,
        radius: 50.0,
        material: moving_sphere_material,
    }));

    let glass: Arc<dyn Material> = Arc::new(Dielectric {
        index_refraction: 0.5,
    });
    objects.push(Arc::new(Sphere {
        center: Vec3::new(260.0, 150.0, 45.0),
        radius: 50.0,
        material: Arc::clone(&glass),
    }));
    objects.push(Arc::new(Sphere {
        center: Vec3::new(0.0, 150.0, 145.0),
        radius: 50.0,
        material: Arc::new(Metal {
            albedo: Vec3::new(0.8, 0.8, 0.9),
            fuzz: 1.0,
        }),
    }));

    let boundary: Arc<dyn Hittable> = Arc::new(Sphere {
        center: Vec3::new(360.0, 150.0, 145.0),
        radius: 70.0,
        material: Arc::clone(&glass),
    });
    objects.push(Arc::clone(&boundary));
    objects.push(Arc::new(ConstantMedium::from_color(
        Arc::clone(&boundary),
        0.2,
        Vec3::new(0.2, 0.4, 0.9),
    )));
    let boundary: Arc<dyn Hittable> = Arc::new(Sphere {
        center: Vec3::zero(),
        radius: 5000.0,
        material: Arc::clone(&glass),
    });
    objects.push(Arc::new(ConstantMedium::from_color(
        Arc::clone(&boundary),
        0.0001,
        Vec3::one(),
    )));

    objects.push(Arc::new(Sphere {
        center: Vec3::new(400.0, 200.0, 400.0),
        radius: 100.0,
        material: Arc::new(Lambertian {
            albedo: Arc::new(ImageTexture::new("earthmap.jpg").unwrap()),
        }),
    }));
    objects.push(Arc::new(Sphere {
        center: Vec3::new(220.0, 280.0, 300.0),
        radius: 80.0,
        material: Arc::new(Lambertian {
            albedo: Arc::new(NoiseTexture::new(0.1)),
        }),
    }));

    let mut boxes: Vec<Arc<dyn Hittable>> = Vec::new();
    let white: Arc<dyn Material> = Arc::new(Lambertian::from_color(Vec3::splat(0.73)));
    let ns = 1000;
    for _ in 0..ns {
        boxes.push(Arc::new(Sphere {
            center: Vec3::random_in_range(0.0, 165.0),
            radius: 10.0,
            material: Arc::clone(&white),
        }))
    }

    objects.push(Arc::new(Translate::new(
        Arc::new(RotateY::new(
            Arc::new(BVHNode::build_tree(&mut boxes, 0.0, 1.0)),
            15.0,
        )),
        Vec3::new(-100.0, 270.0, 395.0),
    )));

    HittableList { objects }
}

pub fn final_scene_camera(default: &CameraConfig) -> CameraConfig {
    CameraConfig {
        vfov: 40.0,
        lookfrom: Vec3::new(478.0, 278.0, -600.0),
        lookat: Vec3::new(278.0, 278.0, 0.0),
        aperture: 0.0,
        ..*default
    }
}
