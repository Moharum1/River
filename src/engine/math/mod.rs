mod WhittedIntegrator;
mod Normal;
mod Vector;
mod Point;
pub mod rays;

use crate::engine::lights::GeneralLight;
use crate::engine::primitives::GeneralPrimitive;
use crate::engine::Scene;

// A Class responsible for rendering the scene
pub(crate) trait Integrator{

    fn render(&self, scene: &Scene<GeneralPrimitive, GeneralLight>);
}


pub(crate) trait Camera{

}