use cgmath::{
    Vector3
};

pub struct Ray {
    pub origin: Vector3<f32>,
    pub direction: Vector3<f32>,
    pub inv_direction: Vector3<f32>,
    pub time: f32
}

impl Ray {
    pub fn new(origin: Vector3<f32>, direction: Vector3<f32>, time: f32) -> Self {
        Self {
            origin,
            direction,
            inv_direction: 1.0 / direction,
            time
        }
    }
    
    pub fn point_at_parameter(&self, t: f32) -> Vector3<f32> {
        self.origin + t * self.direction
    }
}