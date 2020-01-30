use cgmath::Vector3;
use image;

use crate::perlin::Perlin;

pub trait Textured {
    fn value(&self, u: f32, v: f32, p: &Vector3<f32>) -> Vector3<f32>;
}

pub enum Texture {
    Constant(ConstantTexture),
    Checker(CheckerTexture),
    Noise(NoiseTexture),
    Image(ImageTexture)
}

impl Texture {
    pub fn constant(r: f32, g: f32, b: f32) -> Texture {
        Texture::Constant(ConstantTexture { color: Vector3::new(r, g, b) })
    }

    pub fn checker(t0: Texture, t1: Texture) -> Texture {
        Texture::Checker(CheckerTexture::new(t0, t1))
    }

    pub fn noise(scale: f32) -> Texture {
        Texture::Noise(NoiseTexture::new(scale))
    }

    pub fn image(path_str: &str) -> Texture {
        Texture::Image(ImageTexture::new(path_str))
    }
}

impl Textured for Texture {
    fn value(&self, u: f32, v: f32, p: &Vector3<f32>) -> Vector3<f32> {
        match &self {
            Texture::Constant(t) => t.value(u, v, p),
            Texture::Checker(t) => t.value(u, v, p),
            Texture::Noise(t) => t.value(u, v, p),
            Texture::Image(t) => t.value(u, v, p)
        }
    }
}

pub struct ConstantTexture {
    color: Vector3<f32>
}

impl ConstantTexture {
    pub fn new(r:f32, g: f32, b: f32) -> ConstantTexture {
        ConstantTexture {
            color: Vector3::new(r, g, b)
        }
    }
}

impl Textured for ConstantTexture {
    fn value(&self, u: f32, v: f32, p: &Vector3<f32>) -> Vector3<f32> {
        self.color
    }
}

pub struct CheckerTexture {
    odd: Box<Texture>,
    even: Box<Texture>
}

impl CheckerTexture {
    pub fn new(even: Texture, odd: Texture) -> CheckerTexture {
        CheckerTexture {
            even: Box::new(even),
            odd: Box::new(odd)
        }
    }
}

impl Textured for CheckerTexture {
    fn value(&self, u: f32, v: f32, p: &Vector3<f32>) -> Vector3<f32> {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

pub struct NoiseTexture {
    noise: Perlin,
    scale: f32
}

impl NoiseTexture {
    pub fn new(scale: f32) -> NoiseTexture {
        NoiseTexture {
            noise: Perlin::new(),
            scale
        }
    }
}

impl Textured for NoiseTexture {
    fn value(&self, u: f32, v: f32, p: &Vector3<f32>) -> Vector3<f32> {
        Vector3::new(1.0, 1.0, 1.0) * 0.5 * (1.0 + (self.scale * &p.z + 10.0 * self.noise.turb(1.0 * p, 7)).sin())
    }
}

pub struct ImageTexture {
    data: Vec<u8>,
    nx: i32,
    ny: i32
}

impl ImageTexture {
    pub fn new(path_str: &str) -> ImageTexture {
        let path = std::path::Path::new(path_str);
        if let Ok((width, height)) = image::image_dimensions(path) {
            if let Ok(img) = image::open(path) {
                ImageTexture {
                    data: img.raw_pixels(),
                    nx: width as i32,
                    ny: height as i32
                }
            } else {
                ImageTexture {
                    data: vec![255, 255, 0],
                    nx: 1,
                    ny: 1
                }
            }
        } else {
            ImageTexture {
                data: vec![255, 255, 0],
                nx: 1,
                ny: 1
            }
        }
    }
}

impl Textured for ImageTexture {
    fn value(&self, u: f32, v: f32, p: &Vector3<f32>) -> Vector3<f32> {
        let mut i = (u * self.nx as f32) as i32;
        let mut j = ((1.0 - v) * self.ny as f32 - 0.001) as i32; // TODO: eps?
        i = i.max(0).min(self.nx - 1);
        j = j.max(0).min(self.ny - 1);
        let r = self.data[(3 * i + 3 * self.nx * j    ) as usize] as f32 / 255.0;
        let g = self.data[(3 * i + 3 * self.nx * j + 1) as usize] as f32 / 255.0;
        let b = self.data[(3 * i + 3 * self.nx * j + 2) as usize] as f32 / 255.0;
        Vector3::new(r, g, b)
    }
}