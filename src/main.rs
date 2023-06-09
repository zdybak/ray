use std::time::Instant;

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

    pattern::chapter_ten_patterns();

    let elapsed_time = start_time.elapsed();
    println!("Program completed in {:.2?}", elapsed_time);
}
