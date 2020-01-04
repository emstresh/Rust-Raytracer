use cgmath::{ Vector3, InnerSpace, ElementWise };
use rand::prelude::*;
use rayon::prelude::*;

use std::time::Instant;

use crate::camera::Camera;
use crate::hitable::{ Geometry, hit_list };
use crate::ray::Ray;
use crate::material::{ Scattered, Emitter };

const NUM_SAMPLES: i32 = 64;
const MAX_DEPTH: i32 = 16;

fn color(r: Ray, world: &[Geometry], depth: i32) -> Vector3<f32> {
    if let Some(hit) = hit_list(world, &r, 0.001, std::f32::MAX) {
        if depth < MAX_DEPTH {
            let emitted = hit.material.emitted(hit.u, hit.v, &hit.p);
            if let Some(scatter) = hit.material.scatter(r, &hit) {
                return emitted + scatter.attenuation.mul_element_wise(color(scatter.ray, world, depth + 1));
            } else {
                return emitted; // Vector3::new(0.0, 0.0, 0.0);
            }
        }
        return Vector3::new(0.0, 0.0, 0.0);
    } else {
        Vector3::new(0.0, 0.0, 0.0)
        // let unit_direction = r.direction.normalize();
        // let t = 0.5 * (unit_direction.y + 1.0);
        // ((1.0 - t) * Vector3::new(1.0, 1.0, 1.0)) + (t * Vector3::new(0.5, 0.7, 1.0))
    }
}

pub fn draw(camera: Camera, world: Vec<Geometry>, width: usize, height: usize) -> Vec<u32> {
    let now = Instant::now();
    let mut buffer: Vec<u32> = vec![0; width * height];

    let f_width = width as f32;
    let f_height = height as f32;
    let f_samples = NUM_SAMPLES as f32;
    
    buffer.par_chunks_mut(width).enumerate().for_each(|(j, row)| {
        for i in 0..width {
            let mut col = Vector3::new(0.0, 0.0, 0.0);
            for _s in 0..NUM_SAMPLES {
                let u = (i as f32 + random::<f32>()) / f_width;
                let v = 1.0 - ((j as f32 + random::<f32>()) / f_height);

                let r = camera.get_ray(u, v);
                col += color(r, &world[..], 0);
            }
            col /= f_samples;

            let ir = (255.0 * col[0].sqrt()) as u32;
            let ig = (255.0 * col[1].sqrt()) as u32;
            let ib = (255.0 * col[2].sqrt()) as u32;

            row[i] = argb(ir, ig, ib);
        }
    });

    println!("{} seconds to draw scene", now.elapsed().as_secs());
    buffer
}

fn argb(r: u32, g: u32, b: u32) -> u32 {
    255 << 24 | r << 16 | g << 8 | b
}
