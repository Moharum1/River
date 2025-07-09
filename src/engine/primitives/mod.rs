use crate::engine::{Bound3f, Ray, SurfaceInteraction};

pub(crate) trait Primitive{

    // Get the BoundingBox of the scene Geometry
    fn world_bound(&self) -> Bound3f;

    // Check if a ray hit an object and return the interaction details
    fn intersect(&self, ray : &Ray) -> Option<SurfaceInteraction>;

    // Check if there is an intersection along the ray but only return a state
    fn intersect_p(&self, ray : &Ray) -> bool;
}

pub struct GeneralPrimitive{

}

impl Primitive for GeneralPrimitive{
    fn world_bound(&self) -> Bound3f {
        Bound3f{

        }
    }

    fn intersect(&self, ray: &Ray) -> Option<SurfaceInteraction> {
        None
    }

    fn intersect_p(&self, ray: &Ray) -> bool {
        false
    }
}