use cgmath::{ dot, Vector3, InnerSpace };
use rand::prelude::*;

fn permute(p: &mut[u8], n: usize) {
    for i in (0..n as usize).rev() {
        let target = (random::<f32>() * (i + 1) as f32) as usize;
        p.swap(i, target);
    }
}

fn perlin_generate_perm() -> Vec<u8> {
    let mut p = vec![0; 256];
    for i in 0..256 {
        p[i] = i as u8;
    }
    permute(&mut p, 256);
    p
}

fn perlin_generate() -> Vec<Vector3<f32>> {
    let mut p = Vec::with_capacity(256);
    for i in 0..256 {
        p.push(Vector3::new(
            -1.0 + 2.0 * random::<f32>(),
            -1.0 + 2.0 * random::<f32>(),
            -1.0 + 2.0 * random::<f32>()
        ).normalize());
    }
    p
}

// fn perlin_generate() -> Vec<f32> {
//     let mut p = vec![0.0; 256];
//     for i in 0..256 {
//         p[i] = random::<f32>();
//     }
//     p
// }

fn perlin_interpolate(c: &[[[Vector3<f32>; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);
    let mut accum = 0.0;
    for i in 0..2 {
        let f_i = i as f32;
        for j in 0..2 {
            let f_j = j as f32;
            for k in 0..2 {
                let f_k = k as f32;
                let weight_v = Vector3::new(u - f_i, v - f_j, w - f_k);
                accum += (f_i * uu + (1.0 - f_i) * (1.0 - uu)) *
                         (f_j * vv + (1.0 - f_j) * (1.0 - vv)) *
                         (f_k * ww + (1.0 - f_k) * (1.0 - ww)) * dot(c[i][j][k], weight_v);
            }
        }
    }
    accum
}

// fn trilinear_interpolate(c: &[[[f32; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
//     let mut accum = 0.0;
//     for i in 0..2 {
//         let f_i = i as f32;
//         for j in 0..2 {
//             let f_j = j as f32;
//             for k in 0..2 {
//                 let f_k = k as f32;
//                 accum += (f_i * u + (1.0 - f_i)*(1.0 - u)) *
//                          (f_j * v + (1.0 - f_j)*(1.0 - v)) *
//                          (f_k * w + (1.0 - f_k)*(1.0 - w)) * c[i][j][k];
//             }
//         }
//     }
//     accum
// }

pub struct Perlin {
    ranvec: Vec<Vector3<f32>>,
    // ranfloat: Vec<f32>,
    perm_x: Vec<u8>,
    perm_y: Vec<u8>,
    perm_z: Vec<u8>
}

impl Perlin {
    pub fn new() -> Perlin {
        Perlin {
            ranvec: perlin_generate(),
            // ranfloat: perlin_generate(),
            perm_x: perlin_generate_perm(),
            perm_y: perlin_generate_perm(),
            perm_z: perlin_generate_perm()
        }
    }

    pub fn noise(&self, p: Vector3<f32>) -> f32 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();
        
        let i = p.x.floor() as u8;
        let j = p.y.floor() as u8;
        let k = p.z.floor() as u8;

        let mut c = [[[Vector3::new(0.0, 0.0, 0.0); 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let x = i.wrapping_add(di) as usize;
                    let y = j.wrapping_add(dj) as usize;
                    let z = k.wrapping_add(dk) as usize;
                    let idx = self.perm_x[x] ^ self.perm_y[y] ^ self.perm_z[z];
                    c[di as usize][dj as usize][dk as usize] = self.ranvec[idx as usize];
                }
            }
        }

        (perlin_interpolate(&c, u, v, w) + 1.0) * 0.5
    }

    // pub fn turb(&self, p: Vector3<f32>, depth: u8) -> f32 {
    //     let mut accum: f32 = 0.0;
    //     let mut temp_p = p;
    //     let mut weight = 1.0;
    //     for i in 0..depth {
    //         accum += weight * self.noise(temp_p);
    //         weight *= 0.5;
    //         temp_p *= 2.0;
    //     }
    //     accum.abs()
    // }

    // pub fn noise(&self, p: Vector3<f32>) -> f32 {
    //     let mut u = p.x - p.x.floor();
    //     let mut v = p.y - p.y.floor();
    //     let mut w = p.z - p.z.floor();
    //     u = u * u * (3.0 - 2.0 * u);
    //     v = v * v * (3.0 - 2.0 * v);
    //     w = w * w * (3.0 - 2.0 * w);

    //     let i = p.x.floor() as u8;
    //     let j = p.y.floor() as u8;
    //     let k = p.z.floor() as u8;

    //     let mut c = [[[0.0; 2]; 2]; 2];
    //     for di in 0..2 {
    //         for dj in 0..2 {
    //             for dk in 0..2 {
    //                 let x = i.wrapping_add(di) as usize;
    //                 let y = j.wrapping_add(dj) as usize;
    //                 let z = k.wrapping_add(dk) as usize;
    //                 let idx = self.perm_x[x] ^ self.perm_y[y] ^ self.perm_z[z];
    //                 c[di as usize][dj as usize][dk as usize] = self.ranfloat[idx as usize];
    //             }
    //         }
    //     }

    //     trilinear_interpolate(&c, u, v, w)
    // }
}