// An Integrator based on Whitted's ray tracing Alg
// Very accuracte in computing reflected and transmitted light from
// Specular objects like glass, mirrors and water
// struct WhitedIntegrator{
//     max_depth: u32,
// }
//
// impl WhitedIntegrator {
//     fn new(max_depth: u32, camera : &impl Camera) -> WhitedIntegrator {}
// }
//
// impl SamplerIntegrator for WhitedIntegrator {
//     fn render(&self, scene: &Scene<GeneralPrimitive, GeneralLight>) {
//         todo!()
//     }
//
//     fn get_sampler(&self) -> &impl crate::engine::samplers::Sampler {
//         todo!()
//     }
//
//     fn get_camera(&self) -> &impl Camera {
//         todo!()
//     }
//
//     fn preprocess(&self, scene: &Scene<GeneralPrimitive, GeneralLight>) {
//         todo!()
//     }
//
//     fn Li(&self, ray: Ray, scene: Scene<GeneralPrimitive, GeneralLight>, area: crate::engine::samplers::MemoryArea, depth: i32) -> crate::engine::samplers::Spectrum {
//         let mut l;
//         let isec = match scene.intersect(&ray) {
//             Some(isec) => isec,
//             None => {
//                 for light in &scene.lights{
//                     l += light.Le(&ray)
//                 }
//             }
//         };
//
//         let n : Normal3f = isec.shading.n;
//         let wo : Vector3f = isec.wo;
//         isec.compute_scattering_function(ray, area);
//         let l += isec.Le(wo);
//
//         for light in &scene.lights{
//             let li = light.sample_li(isec, self.get_2d, &wi, &pdf, &visibility)
//         }
//
//     }
// }