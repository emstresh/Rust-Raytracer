use cgmath::{ Vector3, ElementWise };
use rand::prelude::*;
use rayon::prelude::*;

use std::time::Instant;

use crate::camera::Camera;
use crate::hitable::{ Geometry, hit_list };
use crate::ray::Ray;
use crate::material::{ Scattered, Emitter };


fn color(r: Ray, world: &[Geometry], depth: i32, max_depth: i32) -> Vector3<f32> {
    if let Some(hit) = hit_list(world, &r, 0.001, std::f32::MAX) {
        if depth < max_depth {
            let emitted = hit.material.emitted(hit.u, hit.v, &hit.p);
            if let Some(scatter) = hit.material.scatter(r, &hit) {
                return emitted + scatter.attenuation.mul_element_wise(color(scatter.ray, world, depth + 1, max_depth));
            } else {
                return emitted;
            }
        }
        return Vector3::new(0.0, 0.0, 0.0);
    } else {
        Vector3::new(0.0, 0.0, 0.0)
    }
}

pub fn draw(camera: Camera, world: Vec<Geometry>, width: usize, height: usize, num_samples: i32, max_depth: i32) -> Vec<u32> {
    let now = Instant::now();
    let mut buffer: Vec<u32> = vec![0; width * height];

    let f_width = width as f32;
    let f_height = height as f32;
    let f_samples = num_samples as f32;
    
    buffer.par_chunks_mut(width).enumerate().for_each(|(j, row)| {
        for i in 0..width {
            let mut col = Vector3::new(0.0, 0.0, 0.0);
            for _s in 0..num_samples {
                let u = (i as f32 + random::<f32>()) / f_width;
                let v = 1.0 - ((j as f32 + random::<f32>()) / f_height);

                let r = camera.get_ray(u, v);
                col += color(r, &world[..], 0, max_depth);
            }
            col /= f_samples;

            let ir = (255.0 * col[0].sqrt()).max(0.0).min(255.0) as u32;
            let ig = (255.0 * col[1].sqrt()).max(0.0).min(255.0) as u32;
            let ib = (255.0 * col[2].sqrt()).max(0.0).min(255.0) as u32;

            row[i] = argb(ir, ig, ib);
        }
    });

    println!("{} seconds to draw scene", now.elapsed().as_secs());
    buffer
}

fn argb(r: u32, g: u32, b: u32) -> u32 {
    255 << 24 | r << 16 | g << 8 | b
}
