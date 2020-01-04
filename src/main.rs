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
// mod rect;
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

    let look_from = Vector3::new(25.0, 5.0, 3.0);
    let look_at = Vector3::new(0.0, 0.0, 0.0);
    let dist_to_focus = 10.0; // (Vector3::new(look_from.x, look_from.y, look_from.z) - look_at).magnitude();
    let aperture = 0.0;
    let camera = camera::Camera::new(look_from, look_at, Vector3::new(0.0, 1.0, 0.0), 20.0, ASPECT, aperture, dist_to_focus, 0.0, 1.0);

    let mut world = world::simple_lights();
    // let bvh = bvh::BvhNode::new(&mut world[..], 0.0, 1.0);
    let buffer = renderer::draw(camera, world, WIDTH, HEIGHT);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer).unwrap();
    }
}
