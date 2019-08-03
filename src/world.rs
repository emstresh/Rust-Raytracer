use cgmath::{ Point3, Vector3, InnerSpace };
use rand::prelude::*;

use std::time::{ Instant };

use crate::hitable::{ Hitable, HitableList };
use crate::material::Material;

pub fn random_scene() -> HitableList {
    let now = Instant::now();
    let mut world = HitableList::new(500);
    world.items.push(Hitable::sphere(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Material::lambertian(
            Vector3::new(0.5, 0.5, 0.5)
        )
    ));

    let temp = Vector3::new(4.0, 0.2, 0.0);
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random::<f32>();
            let center = Vector3::new(
                a as f32 + 0.9 * random::<f32>(),
                0.2,
                b as f32 + 0.9 * random::<f32>()
            );

            if (center - temp).magnitude() > 0.9 {
                if choose_mat < 0.8 {
                    world.items.push(Hitable::sphere(
                        Point3::new(center.x, center.y, center.z),
                        0.2,
                        Material::lambertian(
                            Vector3::new(
                                random::<f32>()*random::<f32>(),
                                random::<f32>()*random::<f32>(),
                                random::<f32>()*random::<f32>()
                            )
                        )
                    ))
                } else if choose_mat < 0.95 {
                    world.items.push(Hitable::sphere(
                        Point3::new(center.x, center.y, center.z),
                        0.2,
                        Material::metal(
                            Vector3::new(
                                0.5 * (1.0 + random::<f32>()),
                                0.5 * (1.0 + random::<f32>()),
                                0.5 * (1.0 + random::<f32>())
                            ),
                            0.5 * random::<f32>()
                        )
                    ))
                } else {
                    world.items.push(Hitable::sphere(
                        Point3::new(center.x, center.y, center.z),
                        0.2,
                        Material::dielectric(1.5)
                    ))
                }
            }
        }
    }

    world.items.push(Hitable::sphere(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        Material::dielectric(1.5)
    ));

    world.items.push(Hitable::sphere(
        Point3::new(-3.0, 1.0, 0.0),
        1.0,
        Material::lambertian(Vector3::new(0.4, 0.2, 0.1))
    ));

    world.items.push(Hitable::sphere(
        Point3::new(3.0, 1.0, 0.0),
        1.0,
        Material::metal(Vector3::new(0.7, 0.6, 0.5), 0.0)
    ));

    println!("{} seconds to generate scene", now.elapsed().as_secs());

    world
}