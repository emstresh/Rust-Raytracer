use cgmath::{ Vector3, InnerSpace };
// use rand::prelude::*;

// use std::time::Instant;

use crate::hitable::{ Geometry };
use crate::material::Material;
use crate::texture::{ Texture };


// pub fn many_spheres() -> Vec<Geometry> {
//     let now = Instant::now();
//     let mut world = Vec::with_capacity(500);
//     world.push(Geometry::sphere(
//         Vector3::new(0.0, -1000.0, 0.0),
//         1000.0,
//         Material::lambertian(
//             Texture::checker(
//                 Texture::constant(0.2, 0.3, 0.1),
//                 Texture::constant(0.9, 0.9, 0.9)
//             )
//         )
//     ));

//     let temp = Vector3::new(4.0, 0.2, 0.0);
//     for a in -11..11 {
//         for b in -11..11 {
//             let choose_mat = random::<f32>();
//             let center = Vector3::new(
//                 a as f32 + 0.9 * random::<f32>(),
//                 0.2,
//                 b as f32 + 0.9 * random::<f32>()
//             );

//             if (center - temp).magnitude() > 0.9 { // diffuse
//                 if choose_mat < 0.8 {
//                     world.push(Geometry::moving_sphere(
//                         Vector3::new(center.x, center.y, center.z),
//                         center + Vector3::new(0.0, 0.5 * random::<f32>(), 0.0),
//                         0.0,
//                         1.0,
//                         0.2,
//                         Material::lambertian(
//                             Texture::constant(
//                                 random::<f32>()*random::<f32>(),
//                                 random::<f32>()*random::<f32>(),
//                                 random::<f32>()*random::<f32>()
//                             )
//                         )
//                     ))
//                 } else if choose_mat < 0.95 { // metallic
//                     world.push(Geometry::sphere(
//                         Vector3::new(center.x, center.y, center.z),
//                         0.2,
//                         Material::metal(
//                             Texture::constant(
//                                 0.5 * (1.0 + random::<f32>()),
//                                 0.5 * (1.0 + random::<f32>()),
//                                 0.5 * (1.0 + random::<f32>())
//                             ),
//                             0.5 * random::<f32>()
//                         )
//                     ))
//                 } else { // dielectric
//                     world.push(Geometry::sphere(
//                         Vector3::new(center.x, center.y, center.z),
//                         0.2,
//                         Material::dielectric(1.5)
//                     ))
//                 }
//             }
//         }
//     }

//     world.push(Geometry::sphere(
//         Vector3::new(0.0, 1.0, 0.0),
//         1.0,
//         Material::dielectric(1.5)
//     ));

//     world.push(Geometry::sphere(
//         Vector3::new(-3.0, 1.0, 0.0),
//         1.0,
//         Material::lambertian(Texture::constant(0.4, 0.2, 0.1))
//     ));

//     world.push(Geometry::sphere(
//         Vector3::new(3.0, 1.0, 0.0),
//         1.0,
//         Material::metal(Texture::constant(0.7, 0.6, 0.5), 0.0)
//     ));

//     println!("{} seconds to generate scene", now.elapsed().as_secs());

//     world
// }

// pub fn two_checker_spheres() -> Vec<Geometry> {
//     let mut world = Vec::with_capacity(2);
//     world.push(Geometry::sphere(
//         Vector3::new(0.0, -10.0, 0.0),
//         10.0,
//         Material::lambertian(
//             Texture::checker(
//                 Texture::constant(0.2, 0.3, 0.1),
//                 Texture::constant(0.9, 0.9, 0.9)
//             )
//         )
//     ));

//     world.push(Geometry::sphere(
//         Vector3::new(0.0, 10.0, 0.0),
//         10.0,
//         Material::lambertian(
//             Texture::checker(
//                 Texture::constant(0.2, 0.3, 0.1),
//                 Texture::constant(0.9, 0.9, 0.9)
//             )
//         )
//     ));

//     world
// }

// pub fn noisy_spheres() -> Vec<Geometry> {
//     let mut world = Vec::with_capacity(2);

//     world.push(Geometry::sphere(
//         Vector3::new(0.0, -1000.0, 0.0),
//         1000.0,
//         Material::lambertian(
//             Texture::noise(5.0)
//         )
//     ));

//     world.push(Geometry::sphere(
//         Vector3::new(0.0, 2.0, 0.0),
//         2.0,
//         Material::lambertian(
//             Texture::noise(5.0)
//         )
//     ));

//     world
// }

// pub fn img_sphere() -> Vec<Geometry> {
//     let mut world = Vec::with_capacity(1);

//     world.push(Geometry::sphere(
//         Vector3::new(0.0, 0.0, 0.0),
//         1.5,
//         Material::lambertian(
//             Texture::image("./8k_mercury.jpg")
//         )
//     ));

//     world
// }

// pub fn simple_lights() -> Vec<Geometry> {
//     let mut world = Vec::with_capacity(4);

//     world.push(Geometry::sphere(
//         Vector3::new(0.0, -1000.0, 0.0),
//         1000.0,
//         Material::lambertian(
//             Texture::noise(4.0)
//         )
//     ));

//     world.push(Geometry::sphere(
//         Vector3::new(0.0, 2.0, 0.0),
//         2.0,
//         Material::lambertian(
//             Texture::noise(4.0)
//         )
//     ));

//     world.push(Geometry::triangle(
//         Vector3::new(3.0, 1.0, -2.0),
//         Vector3::new(3.0, 3.0, -2.0),
//         Vector3::new(5.0, 1.0, -2.0),
//         Material::diffuse_light(
//             Texture::constant(4.0, 4.0, 4.0)
//         )
//     ));

//     world.push(Geometry::triangle(
//         Vector3::new(5.0, 3.0, -2.0),
//         Vector3::new(5.0, 1.0, -2.0),
//         Vector3::new(3.0, 3.0, -2.0),
//         Material::diffuse_light(
//             Texture::constant(4.0, 4.0, 4.0)
//         )
//     ));

//     world
// }

pub fn cornell_box() -> Vec<Geometry> {
    let mut world = Vec::with_capacity(12);

    // green left
    world.push(Geometry::triangle(
        Vector3::new(555.0, 0.0, 0.0),
        Vector3::new(555.0, 0.0, 555.0),
        Vector3::new(555.0, 555.0, 0.0),
        Material::lambertian(
            Texture::constant(0.12, 0.45, 0.15)
        )
    ));
    world.push(Geometry::triangle(
        Vector3::new(555.0, 555.0, 555.0),
        Vector3::new(555.0, 555.0, 0.0),
        Vector3::new(555.0, 0.0, 555.0),
        Material::lambertian(
            Texture::constant(0.12, 0.45, 0.15)
        )
    ));

    // red right
    world.push(Geometry::triangle(
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 555.0, 0.0),
        Vector3::new(0.0, 0.0, 555.0),
        Material::lambertian(
            Texture::constant(0.65, 0.05, 0.05)
        )
    ));
    world.push(Geometry::triangle(
        Vector3::new(0.0, 555.0, 555.0),
        Vector3::new(0.0, 0.0, 555.0),
        Vector3::new(0.0, 555.0, 0.0),
        Material::lambertian(
            Texture::constant(0.65, 0.05, 0.05)
        )
    ));

    // white bottom
    world.push(Geometry::triangle(
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 555.0),
        Vector3::new(555.0, 0.0, 0.0),
        Material::lambertian(
            Texture::constant(0.73, 0.73, 0.73)
        )
    ));
    world.push(Geometry::triangle(
        Vector3::new(555.0, 0.0, 555.0),
        Vector3::new(555.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 555.0),
        Material::lambertian(
            Texture::constant(0.73, 0.73, 0.73)
        )
    ));

    // white top
    world.push(Geometry::triangle(
        Vector3::new(0.0, 555.0, 0.0),
        Vector3::new(555.0, 555.0, 0.0),
        Vector3::new(0.0, 555.0, 555.0),
        Material::lambertian(
            Texture::constant(0.73, 0.73, 0.73)
        )
    ));
    world.push(Geometry::triangle(
        Vector3::new(555.0, 555.0, 555.0),
        Vector3::new(0.0, 555.0, 555.0),
        Vector3::new(555.0, 555.0, 0.0),
        Material::lambertian(
            Texture::constant(0.73, 0.73, 0.73)
        )
    ));

    // white back
    world.push(Geometry::triangle(
        Vector3::new(0.0, 0.0, 555.0),
        Vector3::new(0.0, 555.0, 555.0),
        Vector3::new(555.0, 0.0, 555.0),
        Material::lambertian(
            Texture::constant(0.73, 0.73, 0.73)
        )
    ));
    world.push(Geometry::triangle(
        Vector3::new(555.0, 555.0, 555.0),
        Vector3::new(555.0, 0.0, 555.0),
        Vector3::new(0.0, 555.0, 555.0),
        Material::lambertian(
            Texture::constant(0.73, 0.73, 0.73)
        )
    ));

    // light
    world.push(Geometry::triangle(
        Vector3::new(213.0, 554.0, 227.0),
        Vector3::new(213.0, 554.0, 332.0),
        Vector3::new(343.0, 554.0, 227.0),
        Material::diffuse_light(
            Texture::constant(15.0, 15.0, 15.0)
        )
    ));
    world.push(Geometry::triangle(
        Vector3::new(343.0, 554.0, 332.0),
        Vector3::new(343.0, 554.0, 227.0),
        Vector3::new(213.0, 554.0, 332.0),
        Material::diffuse_light(
            Texture::constant(15.0, 15.0, 15.0)
        )
    ));

    world
}
