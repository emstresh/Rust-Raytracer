use minifb::{ Key, WindowOptions, Window };
use clap::{ Arg, App, SubCommand };
use cgmath::{ Vector3 };
use rand::prelude::*;
use std::env;

mod bbox;
// mod bvh;
mod camera;
mod hitable;
mod material;
mod mesh;
mod moving_sphere;
mod perlin;
mod ray;
mod renderer;
mod sphere;
mod texture;
mod world;
mod util;

fn main() {
    let matches = App::new("raytracer")
                    .version("1.0")
                    .author("emstresh")
                    .about("command line raytracer")
                    .args_from_usage(
                        "-w, --width=[WIDTH] 'Width of output image, in pixels'
                         -h, --height=[HEIGHT] 'Height of output image, in pixels'
                         -s, --samples=[NUM_SAMPLES] 'Number of samples per pixel'
                         -d, --depth=[MAX_DEPTH] 'Maximum number of ray bounces'
                        "
                    )
                    .get_matches();

    let mut width: usize = 640;
    let mut height: usize = 320;
    let mut num_samples: i32 = 256;
    let mut max_depth: i32 = 128;

    if let Some(width_val) = matches.value_of("width") {
        match width_val.parse::<usize>() {
            Ok(w) => width = w,
            Err(e) => panic!("Invalid width argument: {}", e)
        }
    }
    if let Some(height_val) = matches.value_of("height") {
        match height_val.parse::<usize>() {
            Ok(h) => height = h,
            Err(e) => panic!("Invalid height argument: {}", e)
        }
    }
    if let Some(samples_val) = matches.value_of("samples") {
        match samples_val.parse::<i32>() {
            Ok(s) => num_samples = s,
            Err(e) => panic!("Invalid number of samples argument: {}", e)
        }
    }
    if let Some(depth_val) = matches.value_of("depth") {
        match depth_val.parse::<i32>() {
            Ok(d) => max_depth = d,
            Err(e) => panic!("Invalid max depth argument: {}", e)
        }
    }

    let aspect: f32 = width as f32 / height as f32;

    let mut window = Window::new(
        "Raytracer - ESC to exit",
         width,
         height,
         WindowOptions::default()
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });


    let look_from = Vector3::new(278.0, 278.0, -800.0);
    let look_at = Vector3::new(278.0, 278.0, 0.0);
    let dist_to_focus = 10.0; // (Vector3::new(look_from.x, look_from.y, look_from.z) - look_at).magnitude();
    let aperture = 0.0;
    let camera = camera::Camera::new(look_from, look_at, Vector3::new(0.0, 1.0, 0.0), 40.0, aspect, aperture, dist_to_focus, 0.0, 1.0);

    let green_texture = &texture::Texture::constant(0.12, 0.45, 0.15);
    let red_texture = &texture::Texture::constant(0.65, 0.05, 0.05);
    let white_texture = &texture::Texture::constant(0.73, 0.73, 0.73);

    let green_material = &material::Material::lambertian(green_texture);
    let red_material = &material::Material::lambertian(red_texture);
    let white_material = &material::Material::lambertian(white_texture);

    let metal_texture = &texture::Texture::constant(
        0.5 * (1.0 + random::<f32>()),
        0.5 * (1.0 + random::<f32>()),
        0.5 * (1.0 + random::<f32>())
    );

    let metal_material = &material::Material::metal(metal_texture, 0.5 * random::<f32>());

    let emissive_texture = &texture::Texture::constant(15.0, 15.0, 15.0);

    let dielectric_material = &material::Material::dielectric(1.5);
    let emissive_material = &material::Material::diffuse_light(emissive_texture);

    let img_texture = &texture::Texture::image("./img/2k_mars.jpg");
    let mars_material = &material::Material::lambertian(img_texture);

    let noise_texture = &texture::Texture::noise(4.0);
    let noise_material = &material::Material::lambertian(noise_texture);

    let mut world = world::cornell_box(
        green_material,
        red_material,
        white_material,
        metal_material,
        dielectric_material,
        mars_material,
        noise_material,
        emissive_material
    );
    // let bvh = bvh::BvhNode::new(&mut world[..], 0.0, 1.0);
    let buffer = renderer::draw(camera, world, width, height, num_samples, max_depth);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer).unwrap();
    }
}
