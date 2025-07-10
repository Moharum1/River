use crate::engine::math::Point::Point3f;
use crate::engine::math::rays::{BaseRay, Medium};
use crate::engine::math::Vector::Vector3f;


pub(crate) struct Ray{
    origin: Point3f,
    direction: Vector3f,
    t_max : f32,
    time : f32,// For animations
    medium : Option<Medium>,
}

impl BaseRay for Ray{
    fn new(origin: Point3f, direction: Vector3f, t_max: f32, time: f32, medium: Option<Medium>) -> Ray {
        Self{
            origin,
            direction,
            t_max,
            time,
            medium
        }
    }

    fn get_origin(&self) -> Point3f {
         self.origin
    }

    fn get_direction(&self) -> Vector3f {
        self.direction
    }
}





