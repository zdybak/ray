use std::time::Instant;

mod camera;
mod canvas;
mod color;
mod computations;
mod intersection;
mod light;
mod material;
mod matrix;
mod ray;
mod raytuple;
mod sphere;
mod world;

fn main() {
    let start_time = Instant::now();

    camera::chapter_seven_scene();

    let elapsed_time = start_time.elapsed();
    println!("Program completed in {:.2?}", elapsed_time);
}
