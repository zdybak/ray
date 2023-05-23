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
mod shape;
mod world;

fn main() {
    let start_time = Instant::now();

    //camera::chapter_seven_scene();
    use ray::Ray;
    use raytuple::RayTuple;
    use shape::Shape;
    use matrix::Matrix;

    let r = Ray::new(RayTuple::point(0.0, 0.0, -5.0), RayTuple::vector(0.0,0.0,1.0));
    let mut s = Shape::test_shape();
    s.transform = Matrix::scaling(2.0, 2.0, 2.0);
    let _xs = s.intersect(r);
    println!("saved_ray: {:?}", s.saved_ray);

    let elapsed_time = start_time.elapsed();
    println!("Program completed in {:.2?}", elapsed_time);
}
