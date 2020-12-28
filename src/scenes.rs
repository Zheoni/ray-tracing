use crate::camera::CameraConfig;
use crate::constant_medium::ConstantMedium;
use crate::hittable::{Hittable, HittableList};
use crate::material::*;
use crate::object::*;
use crate::texture::*;
use vec3::Vec3;

use rand::Rng;

pub struct Scene {
    pub world: HittableList,
    pub camera_config: CameraConfig,
    pub background_color: Vec3,
}

pub fn get_scenes() -> [&'static str; 11] {
    [
        "spheres",
        "bouncing_spheres",
        "checker_ground",
        "checker_spheres",
        "perlin_spheres",
        "earth",
        "black",
        "simple_light",
        "cornell_box",
        "cornell_smoke",
        "final_scene",
    ]
}
pub fn get_scene_from_name(name: &str) -> Option<Scene> {
    match name {
        "spheres" => Some(random_spheres()),
        "bouncing_spheres" => Some(random_bouncing_spheres()),
        "checker_ground" => Some(random_spheres_checker()),
        "checker_spheres" => Some(checker_spheres()),
        "perlin_spheres" => Some(perlin_spheres()),
        "earth" => Some(earth()),
        "black" => Some(black()),
        "simple_light" => Some(simple_light()),
        "cornell_box" => Some(cornell_box()),
        "cornell_smoke" => Some(cornell_smoke()),
        "final_scene" => Some(final_scene()),
        _ => None,
    }
}

fn default_cam() -> CameraConfig {
    CameraConfig {
        lookfrom: Vec3::new(12.0, 2.0, 3.0),
        lookat: Vec3::new(0.0, 0.0, 0.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
        focus_distance: 10.0,
        vfov: 20.0,
        aperture: 0.1,
        time0: 0.0,
        time1: 1.0,
    }
}

fn random_spheres() -> Scene {
    let mut objects: Vec<Box<dyn Hittable>> = Vec::new();

    // Add the ground
    objects.push(Box::new(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Lambertian::from_color(Vec3::new(0.5, 0.5, 0.5)),
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

                    objects.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Lambertian::from_color(color),
                    }));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Vec3::random_in_range(0.5, 1.0);
                    let fuzz = rand::thread_rng().gen_range(0.0..0.5);

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
        material: Lambertian::from_color(Vec3::new(0.4, 0.2, 0.1)),
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

    Scene {
        world: HittableList { objects },
        camera_config: default_cam(),
        background_color: Vec3::new(0.7, 0.8, 1.0),
    }
}

fn random_bouncing_spheres() -> Scene {
    let mut objects: Vec<Box<dyn Hittable>> = Vec::new();

    // Add the ground
    objects.push(Box::new(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Lambertian::from_color(Vec3::new(0.5, 0.5, 0.5)),
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

                    objects.push(Box::new(MovingSphere {
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
                    let fuzz = rand::thread_rng().gen_range(0.0..0.5);

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
        material: Lambertian::from_color(Vec3::new(0.4, 0.2, 0.1)),
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

    Scene {
        world: HittableList { objects },
        camera_config: default_cam(),
        background_color: Vec3::new(0.7, 0.8, 1.0),
    }
}

fn random_spheres_checker() -> Scene {
    let mut objects: Vec<Box<dyn Hittable>> = Vec::new();

    // Add the ground
    let cheker = CheckerTexture::from_colors(Vec3::new(0.2, 0.3, 0.1), Vec3::new(0.9, 0.9, 0.9));
    objects.push(Box::new(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Lambertian { albedo: cheker },
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

                    objects.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Lambertian::from_color(color),
                    }));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Vec3::random_in_range(0.5, 1.0);
                    let fuzz = rand::thread_rng().gen_range(0.0..0.5);

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
        material: Lambertian::from_color(Vec3::new(0.4, 0.2, 0.1)),
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

    Scene {
        world: HittableList { objects },
        camera_config: default_cam(),
        background_color: Vec3::new(0.7, 0.8, 1.0),
    }
}

fn aperture_0() -> CameraConfig {
    CameraConfig {
        aperture: 0.0,
        ..default_cam()
    }
}

fn checker_spheres() -> Scene {
    let mut objects: Vec<Box<dyn Hittable>> = Vec::new();

    let checker = CheckerTexture::from_colors(Vec3::new(0.2, 0.3, 0.1), Vec3::new(0.9, 0.9, 0.9));
    let material = Lambertian { albedo: checker };
    objects.push(Box::new(Sphere {
        center: Vec3::new(0.0, -10.0, 0.0),
        radius: 10.0,
        material: material.clone(),
    }));
    objects.push(Box::new(Sphere {
        center: Vec3::new(0.0, 10.0, 0.0),
        radius: 10.0,
        material,
    }));

    Scene {
        world: HittableList { objects },
        camera_config: aperture_0(),
        background_color: Vec3::new(0.7, 0.8, 1.0),
    }
}

fn perlin_spheres() -> Scene {
    let mut objects: Vec<Box<dyn Hittable>> = Vec::new();

    let texture = NoiseTexture::new(4.0);
    let material = Lambertian { albedo: texture };

    objects.push(Box::new(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: material.clone(),
    }));

    objects.push(Box::new(Sphere {
        center: Vec3::new(0.0, 2.0, 0.0),
        radius: 2.0,
        material,
    }));

    Scene {
        world: HittableList { objects },
        camera_config: aperture_0(),
        background_color: Vec3::new(0.7, 0.8, 1.0),
    }
}

fn earth() -> Scene {
    let earth_texture = ImageTexture::new("earthmap.jpg").unwrap();
    let earth_surface = Lambertian {
        albedo: earth_texture,
    };
    let globe = Box::new(Sphere {
        center: Vec3::zero(),
        radius: 2.0,
        material: earth_surface,
    });

    Scene {
        world: HittableList {
            objects: vec![globe],
        },
        camera_config: aperture_0(),
        background_color: Vec3::new(0.7, 0.8, 1.0),
    }
}

fn black() -> Scene {
    Scene {
        world: HittableList {
            objects: Vec::new(),
        },
        camera_config: default_cam(),
        background_color: Vec3::zero(),
    }
}

fn simple_light_camera() -> CameraConfig {
    CameraConfig {
        lookfrom: Vec3::new(26.0, 3.0, 6.0),
        lookat: Vec3::new(0.0, 2.0, 0.0),
        ..default_cam()
    }
}

fn simple_light() -> Scene {
    let mut objects: Vec<Box<dyn Hittable>> = Vec::new();

    let marble = Lambertian {
        albedo: NoiseTexture::new(4.0),
    };

    objects.push(Box::new(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: marble.clone(),
    }));
    objects.push(Box::new(Sphere {
        center: Vec3::new(0.0, 2.0, 0.0),
        radius: 2.0,
        material: marble,
    }));

    let difflight = DiffuseLight::from_color(Vec3::splat(4.0));
    objects.push(Box::new(Rect {
        in_plane: XY,
        a0: 3.0,
        a1: 5.0,
        b0: 1.0,
        b1: 3.0,
        k: -2.0,
        material: difflight,
    }));

    Scene {
        world: HittableList { objects },
        camera_config: simple_light_camera(),
        background_color: Vec3::zero(),
    }
}

fn cornell_box() -> Scene {
    let mut objects: Vec<Box<dyn Hittable>> = Vec::new();

    let red = Lambertian::from_color(Vec3::new(0.65, 0.05, 0.05));
    let white = Lambertian::from_color(Vec3::splat(0.73));
    let green = Lambertian::from_color(Vec3::new(0.12, 0.45, 0.15));
    let light = DiffuseLight::from_color(Vec3::splat(15.0));

    // Sides
    objects.push(Box::new(Rect {
        in_plane: YZ,
        a0: 0.0,
        a1: 555.0,
        b0: 0.0,
        b1: 555.0,
        k: 555.0,
        material: green,
    }));
    objects.push(Box::new(Rect {
        in_plane: YZ,
        a0: 0.0,
        a1: 555.0,
        b0: 0.0,
        b1: 555.0,
        k: 0.0,
        material: red,
    }));
    objects.push(Box::new(Rect {
        in_plane: XZ,
        a0: 213.0,
        a1: 343.0,
        b0: 227.0,
        b1: 332.0,
        k: 554.0,
        material: light,
    }));
    objects.push(Box::new(Rect {
        in_plane: XZ,
        a0: 0.0,
        a1: 555.0,
        b0: 0.0,
        b1: 555.0,
        k: 0.0,
        material: white.clone(),
    }));
    objects.push(Box::new(Rect {
        in_plane: XZ,
        a0: 0.0,
        a1: 555.0,
        b0: 0.0,
        b1: 555.0,
        k: 555.0,
        material: white.clone(),
    }));
    objects.push(Box::new(Rect {
        in_plane: XY,
        a0: 0.0,
        a1: 555.0,
        b0: 0.0,
        b1: 555.0,
        k: 555.0,
        material: white.clone(),
    }));

    // Blocks
    let box1 = Block::new(Vec3::zero(), Vec3::new(165.0, 330.0, 165.0), white.clone());
    let box1 = RotateY::new(box1, 15.0);
    let box1 = Translate::new(box1, Vec3::new(265.0, 0.0, 295.0));
    let box1 = Box::new(box1);
    objects.push(box1);

    let box2 = Block::new(Vec3::zero(), Vec3::new(165.0, 165.0, 165.0), white);
    let box2 = RotateY::new(box2, -18.0);
    let box2 = Translate::new(box2, Vec3::new(130.0, 0.0, 65.0));
    let box2 = Box::new(box2);
    objects.push(box2);

    Scene {
        world: HittableList { objects },
        camera_config: cornell_box_camera(),
        background_color: Vec3::zero(),
    }
}

fn cornell_box_camera() -> CameraConfig {
    CameraConfig {
        lookfrom: Vec3::new(278.0, 278.0, -800.0),
        lookat: Vec3::new(278.0, 270.0, 0.0),
        vfov: 40.0,
        aperture: 0.0,
        ..default_cam()
    }
}

fn cornell_smoke() -> Scene {
    let mut cb = cornell_box();
    cb.world.objects.pop().unwrap();
    cb.world.objects.pop().unwrap();

    let white = Lambertian::from_color(Vec3::splat(0.73));

    let box1 = Block::new(Vec3::zero(), Vec3::new(165.0, 330.0, 165.0), white.clone());
    let box1 = RotateY::new(box1, 15.0);
    let box1 = Translate::new(box1, Vec3::new(265.0, 0.0, 295.0));

    let box2 = Block::new(Vec3::zero(), Vec3::new(165.0, 165.0, 165.0), white);
    let box2 = RotateY::new(box2, -18.0);
    let box2 = Translate::new(box2, Vec3::new(130.0, 0.0, 65.0));

    cb.world.objects.push(Box::new(ConstantMedium::from_color(
        box1,
        0.01,
        Vec3::zero(),
    )));
    cb.world.objects.push(Box::new(ConstantMedium::from_color(
        box2,
        0.01,
        Vec3::one(),
    )));
    cb
}

fn final_scene() -> Scene {
    use crate::bvh::BVH;
    let mut rng = rand::thread_rng();
    let mut boxes: Vec<Box<dyn Hittable>> = Vec::new();

    let ground = Lambertian::from_color(Vec3::new(0.48, 0.83, 0.53));

    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = rng.gen_range(1.0..101.0);
            let z1 = z0 + w;

            boxes.push(Box::new(Block::new(
                Vec3::new(x0, y0, z0),
                Vec3::new(x1, y1, z1),
                ground.clone(),
            )));
        }
    }

    let mut objects: Vec<Box<dyn Hittable>> = Vec::new();

    objects.push(Box::new(BVH::build(boxes, 0.0, 1.0)));

    let light = DiffuseLight::from_color(Vec3::splat(7.0));
    objects.push(Box::new(Rect {
        in_plane: XZ,
        a0: 123.0,
        a1: 423.0,
        b0: 147.0,
        b1: 412.0,
        k: 445.0,
        material: light,
    }));

    let center0 = Vec3::new(400.0, 400.0, 200.0);
    let center1 = center0 + Vec3::new(30.0, 0.0, 0.0);
    let moving_sphere_material = Lambertian::from_color(Vec3::new(0.7, 0.3, 0.1));
    objects.push(Box::new(MovingSphere {
        center0,
        center1,
        time0: 0.0,
        time1: 1.0,
        radius: 50.0,
        material: moving_sphere_material,
    }));

    let glass = Dielectric {
        index_refraction: 0.5,
    };
    objects.push(Box::new(Sphere {
        center: Vec3::new(260.0, 150.0, 45.0),
        radius: 50.0,
        material: glass.clone(),
    }));
    objects.push(Box::new(Sphere {
        center: Vec3::new(0.0, 150.0, 145.0),
        radius: 50.0,
        material: Metal {
            albedo: Vec3::new(0.8, 0.8, 0.9),
            fuzz: 1.0,
        },
    }));

    let boundary = Sphere {
        center: Vec3::new(360.0, 150.0, 145.0),
        radius: 70.0,
        material: glass.clone(),
    };
    objects.push(Box::new(boundary.clone()));
    objects.push(Box::new(ConstantMedium::from_color(
        boundary,
        0.2,
        Vec3::new(0.2, 0.4, 0.9),
    )));
    let boundary = Sphere {
        center: Vec3::zero(),
        radius: 5000.0,
        material: glass,
    };
    objects.push(Box::new(ConstantMedium::from_color(
        boundary,
        0.0001,
        Vec3::one(),
    )));

    objects.push(Box::new(Sphere {
        center: Vec3::new(400.0, 200.0, 400.0),
        radius: 100.0,
        material: Lambertian {
            albedo: ImageTexture::new("earthmap.jpg").unwrap(),
        },
    }));
    objects.push(Box::new(Sphere {
        center: Vec3::new(220.0, 280.0, 300.0),
        radius: 80.0,
        material: Lambertian {
            albedo: NoiseTexture::new(0.1),
        },
    }));

    let mut boxes: Vec<Box<dyn Hittable>> = Vec::new();
    let white = Lambertian::from_color(Vec3::splat(0.73));
    let ns = 1000;
    for _ in 0..ns {
        boxes.push(Box::new(Sphere {
            center: Vec3::random_in_range(0.0, 165.0),
            radius: 10.0,
            material: white.clone(),
        }))
    }

    objects.push(Box::new(Translate::new(
        RotateY::new(BVH::build(boxes, 0.0, 1.0), 15.0),
        Vec3::new(-100.0, 270.0, 395.0),
    )));

    Scene {
        world: HittableList { objects },
        camera_config: final_scene_camera(),
        background_color: Vec3::zero(),
    }
}

fn final_scene_camera() -> CameraConfig {
    CameraConfig {
        vfov: 40.0,
        lookfrom: Vec3::new(478.0, 278.0, -600.0),
        lookat: Vec3::new(278.0, 278.0, 0.0),
        aperture: 0.0,
        ..default_cam()
    }
}
