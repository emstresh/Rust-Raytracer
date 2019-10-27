use cgmath::Vector3;

use crate::perlin::Perlin;

pub trait Textured {
    fn value(&self, u: f32, v: f32, p: &Vector3<f32>) -> Vector3<f32>;
}

pub enum Texture {
    Constant(ConstantTexture),
    Checker(CheckerTexture),
    Noise(NoiseTexture)
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
}

impl Textured for Texture {
    fn value(&self, u: f32, v: f32, p: &Vector3<f32>) -> Vector3<f32> {
        match &self {
            Texture::Constant(t) => t.value(u, v, p),
            Texture::Checker(t) => t.value(u, v, p),
            Texture::Noise(t) => t.value(u, v, p)
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
        // Vector3::new(1.0, 1.0, 1.0) * ((1.0 + self.noise.noise(self.scale * p)) * 0.5)
        // Vector3::new(1.0, 1.0, 1.0) * self.noise.turb(self.scale * p, 7)
        Vector3::new(1.0, 1.0, 1.0) * 0.5 * (1.0 + (self.scale * &p.z + 10.0 * self.noise.turb(1.0 * p, 7)).sin())
    }
}