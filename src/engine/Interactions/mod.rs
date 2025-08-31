mod surface_interaction;

use crate::engine::math::Normal::Normal3f;
use crate::engine::math::Point::Point3f;
use crate::engine::math::Vector::{Vector3, Vector3f};

trait MediumInterface{

}

pub trait Interactions{
    fn new(point: Point3f, normal: Normal3f, point_error: Vector3f, wo: Vector3f, medium_interface : Box<dyn MediumInterface>) -> Self;

    fn is_surface_interaction(&self, n: Normal3f) -> bool {
        n != Normal3f::default()
    }
}