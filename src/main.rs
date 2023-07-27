use std::time::Instant;

use crate::world::World;

mod camera;
mod canvas;
mod color;
mod computations;
mod intersection;
mod light;
mod material;
mod matrix;
mod pattern;
mod ray;
mod raytuple;
mod shape;
mod world;

fn main() {
    let start_time = Instant::now();

    World::chapter_eleven_reflect();

    let elapsed_time = start_time.elapsed();
    println!("Program completed in {:.2?}", elapsed_time);
}
