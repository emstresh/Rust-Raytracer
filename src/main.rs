use minifb::{ Key, WindowOptions, Window };
use cgmath::{ Point3, Vector3, InnerSpace };

mod camera;
mod hitable;
mod material;
mod ray;
mod renderer;
mod sphere;
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

    let look_from = Point3::new(16.0, 2.0, 4.0);
    let look_at = Vector3::new(0.0, 0.0, 0.0);
    let dist_to_focus = (Vector3::new(look_from.x, look_from.y, look_from.z) - look_at).magnitude();
    let aperture = 0.2;
    let camera = camera::Camera::new(look_from, look_at, Vector3::new(0.0, 1.0, 0.0), 15.0, ASPECT, aperture, dist_to_focus);

    let world = world::random_scene();
    let buffer = renderer::draw(camera, world, WIDTH, HEIGHT);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer).unwrap();
    }
}
