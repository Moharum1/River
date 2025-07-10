use crate::engine::math::Point::Point3f;
use crate::engine::math::Vector::Vector3f;

pub mod Ray;
mod ray_differential;

struct Medium{

}


trait BaseRay{
    fn new(origin : Point3f, direction : Vector3f, t_max : f32, time : f32, medium: Option<Medium>) -> Self;

    fn get_origin(&self) -> Point3f;

    fn get_direction(&self) -> Vector3f;

    fn point_at(&self, t : f32) -> Point3f{
        self.get_origin() + self.get_direction() * t
    }
}