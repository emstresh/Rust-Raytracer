use minifb::{ Key, WindowOptions, Window };
use cgmath::{ Vector3 };

mod bbox;
// mod bvh;
mod camera;
mod hitable;
mod material;
mod moving_sphere;
mod perlin;
mod ray;
mod renderer;
mod sphere;
mod texture;
mod triangle;
mod world;
mod util;

const WIDTH: usize = 640;
const HEIGHT: usize = 320;
const ASPECT: f32 = WIDTH as f32 / HEIGHT as f32;

fn main() {
    let mut window = Window::new(
        "Raytracer - ESC to exit",
         WIDTH,
         HEIGHT,
         WindowOptions::default()
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let look_from = Vector3::new(278.0, 278.0, -800.0);
    let look_at = Vector3::new(278.0, 278.0, 0.0);
    let dist_to_focus = 10.0; // (Vector3::new(look_from.x, look_from.y, look_from.z) - look_at).magnitude();
    let aperture = 0.0;
    let camera = camera::Camera::new(look_from, look_at, Vector3::new(0.0, 1.0, 0.0), 40.0, ASPECT, aperture, dist_to_focus, 0.0, 1.0);

    let green_material = &material::Material::lambertian(texture::Texture::constant(0.12, 0.45, 0.15));
    let red_material = &material::Material::lambertian(texture::Texture::constant(0.65, 0.05, 0.05));
    let white_material = &material::Material::lambertian(texture::Texture::constant(0.73, 0.73, 0.73));
    let emissive_material = &material::Material::diffuse_light(texture::Texture::constant(15.0, 15.0, 15.0));

    let mut world = world::cornell_box(green_material, red_material, white_material, emissive_material);
    // let bvh = bvh::BvhNode::new(&mut world[..], 0.0, 1.0);
    let buffer = renderer::draw(camera, world, WIDTH, HEIGHT);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer).unwrap();
    }
}
