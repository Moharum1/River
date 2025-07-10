use crate::engine::math::Point::Point3f;
use crate::engine::math::rays::{BaseRay, Medium};
use crate::engine::math::Vector::Vector3f;

struct RayDifferential{
    origin: Point3f,
    direction: Vector3f,
    t_max : f32,
    time : f32,// For animations
    medium : Option<Medium>,

    is_differential : bool,
    rx_orig : Option<Point3f>,
    rx_direction : Option<Vector3f>,
    ry_orig : Option<Point3f>,
    ry_direction : Option<Vector3f>,
}

impl BaseRay for RayDifferential {
    fn new(origin: Point3f, direction: Vector3f, t_max: f32, time: f32, medium: Option<Medium>) -> Self {
        Self{
            origin,
            direction,
            t_max,
            time,
            medium,
            is_differential: false,
            rx_orig: None,
            rx_direction: None,
            ry_orig: None,
            ry_direction: None,
        }
    }

    fn get_origin(&self) -> Point3f {
        self.origin
    }

    fn get_direction(&self) -> Vector3f {
        self.direction
    }
}

impl RayDifferential {

    /// Update the differential rays for an estimated sample spacing of s
    pub fn scale_differential(&mut self, s:f32){
        self.rx_orig = Some(self.origin + (self.rx_orig.unwrap() - self.origin) * s);
        self.ry_orig = Some(self.origin + (self.ry_orig.unwrap() - self.origin) * s);
        self.rx_direction = Some(self.direction + (self.rx_direction.unwrap() - self.direction) * s);
        self.ry_direction = Some(self.direction + (self.ry_direction.unwrap() - self.direction) * s);
    }
}


