use crate::engine::primitives::Primitive;
use crate::engine::lights::Light;

mod math;
mod samplers;
mod lights;
mod primitives;

// Primitive Describe a Shape Geometry and it's Material



struct Ray{

}

struct SurfaceInteraction{

}

struct Bound3f{

}

pub struct Bound2i{

}

pub struct Vector2i{

}

pub struct Point2i{
    x : f32,
    y : f32,
}

struct Scene<Primitives, Lights>
where Lights: Light, Primitives: Primitive
{
    lights : Vec<Lights>,
    aggregate : Primitives,
    world_bound : Bound3f
}


impl<Primitives : Primitive, Lights : Light> Scene<Primitives, Lights> {
    pub fn new(aggregate : Primitives, lights : Vec<Lights>) -> Scene<Primitives, Lights> {
        let data = Self{
            lights,
            world_bound: aggregate.world_bound(),
            aggregate,
        };

        for light in &data.lights {
            light.preprocess()
        }
        data
    }

    pub fn intersect(&self, ray : &Ray) -> Option<SurfaceInteraction> {
        self.aggregate.intersect(ray)
    }

    pub fn intersect_p(&self, ray : &Ray) -> bool {
        self.aggregate.intersect_p(ray)
    }
}