use crate::engine::lights::GeneralLight;
use crate::engine::math::Camera;
use crate::engine::primitives::GeneralPrimitive;
use crate::engine::{Bound2i, Point2i, Ray, Scene};

pub trait SamplerIntegrator : crate::engine::math::Integrator {
    fn render(&self, scene: &Scene<GeneralPrimitive, GeneralLight>){
        // self.preprocess(scene);
        //
        // // Compute Number of tiles
        // let sample_bounds = self.get_camera()
        //     .get_film()
        //     .get_sample_bound();
        // let sample_extent = sample_bounds
        //     .diagonal();
        // const TILE_SIZE : i8 = 16;
        // let n_tiles = Point2i{
        //     x : (sample_extent.x + TILE_SIZE - 1) / TILE_SIZE,
        //     y : (sample_extent.y + TILE_SIZE - 1) / TILE_SIZE,
        // };
        //
        // let seed = tile.y * n_tiles.x + tile.x;
        // let tile_sampler = Sampler::Clone(seed);
        //
        // let x0 = sample_bounds.pMin.x + tile.x * TILE_SIZE;
        // let x1 = (x0 + TILE_SIZE, sample_bounds.pMax.x).min();
        // let y0 = sample_bounds.pMin.y + tile.y * TILE_SIZE;
        // let y1 = (y0 + TILE_SIZE, sample_bounds.pMax.y).min();
        // let tile_bounds = Bound2i{
        //     x : Point2i::new(x0, y0),
        //     y : Point2i::new(x1, y1)
        // };
        //
        // let film_tile = self.get_camera()
        //     .get_film_tile(tile_bounds);
        //
        // for (pixel in tile_bounds){
        //     let camera_sample = tile_sampler.get_camera_sampler(pixel);
        //     let (ray_weight, ray) = self.get_camera()
        //         .generate_ray_differentail();
        //     ray.scale_differentials(1/(tile_sampler.samples_per_pixel).sqrt())
        //
        //     if (ray_weight > 0.0){
        //         spectrum = tile_sampler.Li(ray, scene, area, 0)
        //     }
        //     film_tile.add_sample(camera_sample.pFilm, spectrum, ray_weight)
        //
        //}

    }

    fn get_sampler(&self) -> &impl Sampler;

    fn get_camera(&self) -> &impl Camera;

    fn preprocess(&self, scene: &Scene<GeneralPrimitive, GeneralLight>);

    fn Li(&self, ray : Ray, scene: Scene<GeneralPrimitive, GeneralLight>, area : MemoryArea, depth : i32) -> Spectrum;
}

struct Spectrum{

}

trait Sampler{

}


struct MemoryArea{

}
