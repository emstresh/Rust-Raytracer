use crate::ray::Ray;
use crate::hitable::HitRecord;
use crate::texture::{ Texture, Textured };
use crate::util;

use cgmath::{
    Vector3,
    dot,
    InnerSpace
};

pub struct Lambertian {
    pub albedo: Texture
}

pub struct Metal {
    pub albedo: Texture,
    pub fuzz: f32
}

pub struct Dielectric {
    pub ref_idx: f32
}

pub struct Scatter {
    pub attenuation: Vector3<f32>,
    pub ray: Ray
}

pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric)
}

impl Material {
    pub fn lambertian(albedo: Texture) -> Material {
        Material::Lambertian(Lambertian { albedo })
    }

    pub fn metal(albedo: Texture, f: f32) -> Material {
        Material::Metal(Metal { albedo, fuzz: f.min(1.0) })
    }

    pub fn dielectric(ref_idx: f32) -> Material {
        Material::Dielectric(Dielectric { ref_idx })
    }

    pub fn scatter(&self, r_in: Ray, hit: &HitRecord) -> Option<Scatter> {
        match &hit.material {
            Material::Lambertian(l) => l.scatter(r_in, &hit),
            Material::Metal(m) => m.scatter(r_in, &hit),
            Material::Dielectric(d) => d.scatter(r_in, &hit)
        }
    }
}

impl Lambertian {
    fn scatter(&self, r_in: Ray, hit: &HitRecord) -> Option<Scatter> {
        let target = hit.p + hit.normal + util::random_in_unit_sphere();
        Some(Scatter {
            attenuation: self.albedo.value(0.0, 0.0, &hit.p),
            ray: Ray::new(hit.p, target - hit.p, r_in.time)
        })
    }
}

impl Metal {
    fn scatter(&self, r_in: Ray, hit: &HitRecord) -> Option<Scatter> {
        let reflected = reflect(r_in.direction.normalize(), hit.normal);
        if dot(reflected, hit.normal) > 0.0 {
            Some(Scatter {
                attenuation: self.albedo.value(0.0, 0.0, &hit.p),
                ray: Ray::new(
                    hit.p,
                    reflected + self.fuzz * util::random_in_unit_sphere(),
                    r_in.time
                )
            })
        } else {
            None
        }
    }
}

impl Dielectric {
    fn scatter(&self, r_in: Ray, hit: &HitRecord) -> Option<Scatter> {
        let dot_prod = dot(r_in.direction, hit.normal);

        let (outward_normal, ni_over_nt, cosine) = if dot_prod > 0.0 {
            (
                -hit.normal,
                self.ref_idx,
                self.ref_idx * dot_prod / r_in.direction.magnitude()
            )
        } else {
            (
                hit.normal,
                1.0 / self.ref_idx,
                -dot_prod / r_in.direction.magnitude()
            )
        };

        let attenuation = Vector3::new(1.0, 1.0, 1.0);
        let reflected = reflect(r_in.direction, hit.normal);

        if let Some(refracted) = refract(r_in.direction, outward_normal, ni_over_nt) {
            let reflection_prob = schlick(cosine, self.ref_idx);
            let out_dir = if rand::random::<f32>() < reflection_prob {
                reflected
            } else {
                refracted
            };

            Some(Scatter {
                attenuation,
                ray: Ray::new(
                    hit.p,
                    out_dir,
                    r_in.time
                )
            })
        } else {
            Some(Scatter {
                attenuation,
                ray: Ray::new(
                    hit.p,
                    reflected,
                    r_in.time
                )
            })
        }
    }
}

pub fn reflect(v: Vector3<f32>, n: Vector3<f32>) -> Vector3<f32> {
    v - 2.0 * dot(v, n) * n
}

pub fn refract(v: Vector3<f32>, n: Vector3<f32>, ni_over_nt: f32) -> Option<Vector3<f32>> {
    let uv = v.normalize();
    let dt = dot(uv, n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        Some(ni_over_nt * (uv - n * dt) - n * discriminant.sqrt())
    } else {
        None
    }
}

pub fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}