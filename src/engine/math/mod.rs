mod WhittedIntegrator;
mod Normal;
mod Vector;
mod Point;
pub mod rays;
mod BoundingBox;
mod Transformations;

use std::ops::{Add, Mul, Sub};
use std::process::Output;
use num_traits::{FromPrimitive, Signed};
use crate::engine::lights::GeneralLight;
use crate::engine::primitives::GeneralPrimitive;
use crate::engine::Scene;

// A Class responsible for rendering the scene
pub(crate) trait Integrator{

    fn render(&self, scene: &Scene<GeneralPrimitive, GeneralLight>);
}


pub(crate) trait Camera{

}

pub fn lerp<T>(t : T, v1 : T, v2 : T) -> T
where T : Copy + Signed + FromPrimitive + Sub<Output = T> + Add<Output = T> + Mul<Output = T>,
{
     (T::from_u8(1).unwrap() - t) * v1 + t * v2
}