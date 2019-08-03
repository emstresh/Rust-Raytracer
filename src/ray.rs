use cgmath::{
    Vector3,
    Point3
};

pub struct Ray {
    pub origin: Point3<f32>,
    pub direction: Vector3<f32>,
    pub inv_direction: Vector3<f32>
}

impl Ray {
    pub fn new(origin: Point3<f32>, direction: Point3<f32>) -> Self {
        Self {
            origin,
            direction,
            inv_direction: 1.0 / direction
        }
    }
    
    pub fn point_at_parameter(&self, t: f32) -> Point3<f32> {
        self.origin + t * self.direction
    }
}