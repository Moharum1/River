use crate::engine::Point2i;

// Break an image into a sequence of tiles to be processes on multi cores
pub fn parallel_for_2d(func : fn(tile : Point2i) -> i32, n_tiles : i32) -> i32 {
    n_tiles
}

