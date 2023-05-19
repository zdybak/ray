#![allow(dead_code)]
use crate::canvas::Canvas;
use crate::color::Color;
use crate::intersection::Intersection;
use crate::light::Light;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::raytuple::RayTuple;
use crate::shape::{Shape, ShapeType};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: RayTuple,
    pub direction: RayTuple,
}

impl Ray {
    pub fn new(origin: RayTuple, direction: RayTuple) -> Self {
        Self { origin, direction }
    }

    pub fn position(self, t: f64) -> RayTuple {
        self.origin + self.direction * t
    }

    pub fn transform(self, m: Matrix) -> Self {
        Self {
            origin: m * self.origin,
            direction: m * self.direction,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_and_query_ray() {
        let r = Ray::new(
            RayTuple::point(1.0, 2.0, 3.0),
            RayTuple::vector(4.0, 5.0, 6.0),
        );

        assert_eq!(r.origin, RayTuple::point(1.0, 2.0, 3.0));
        assert_eq!(r.direction, RayTuple::vector(4.0, 5.0, 6.0));
    }

    #[test]
    fn compute_point_from_distance() {
        let r = Ray::new(
            RayTuple::point(2.0, 3.0, 4.0),
            RayTuple::vector(1.0, 0.0, 0.0),
        );

        assert_eq!(r.position(0.0), RayTuple::point(2.0, 3.0, 4.0));
        assert_eq!(r.position(1.0), RayTuple::point(3.0, 3.0, 4.0));
        assert_eq!(r.position(-1.0), RayTuple::point(1.0, 3.0, 4.0));
        assert_eq!(r.position(2.5), RayTuple::point(4.5, 3.0, 4.0));
    }

    #[test]
    fn translate_ray() {
        let r = Ray::new(
            RayTuple::point(1.0, 2.0, 3.0),
            RayTuple::vector(0.0, 1.0, 0.0),
        );
        let m = Matrix::translation(3.0, 4.0, 5.0);
        let r2 = r.transform(m);

        assert_eq!(r2.origin, RayTuple::point(4.0, 6.0, 8.0));
        assert_eq!(r2.direction, RayTuple::vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn scale_ray() {
        let r = Ray::new(
            RayTuple::point(1.0, 2.0, 3.0),
            RayTuple::vector(0.0, 1.0, 0.0),
        );
        let m = Matrix::scaling(2.0, 3.0, 4.0);
        let r2 = r.transform(m);

        assert_eq!(r2.origin, RayTuple::point(2.0, 6.0, 12.0));
        assert_eq!(r2.direction, RayTuple::vector(0.0, 3.0, 0.0));
    }
}

pub fn chapter_five_raysphere() {
    let canvas_pixels = 100;
    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    let color = Color::new(1.0, 0.0, 0.0);
    let shape = Shape::new(ShapeType::Sphere);

    //test shape transforms NOTE: change let shape to a mutable Sphere for these.
    //shape.set_transform(Matrix::scaling(1.0, 0.5, 1.0));
    //shape.set_transform(Matrix::scaling(0.5, 1.0, 1.0));
    //shape.set_transform(Matrix::rotation_z(PI / 4.0) * Matrix::scaling(0.5,1.0,1.0));
    //shape.set_transform(Matrix::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0) * Matrix::scaling(0.5,1.0,1.0));

    let ray_origin = RayTuple::point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;

    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.0;

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * y as f64;
        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * x as f64;
            let position = RayTuple::point(world_x, world_y, wall_z);

            let r = Ray::new(ray_origin, (position - ray_origin).normalize());
            let xs = shape.intersect(r);

            let h = Intersection::hit(xs);
            match h {
                Some(_inter) => {
                    canvas.write_pixel(x, y, color);
                }
                None => continue,
            };
        }
    }
    canvas.save_ppm("chapter5sphere.ppm");
}

pub fn chapter_six_lighting() {
    let canvas_pixels = 2160;
    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);

    //ch6 new: color is now determined by calculated material/light
    //let color = Color::new(1.0, 0.0, 0.0);

    //ch6 new: assign a material to the sphere
    let mut shape = Shape::new(ShapeType::Sphere);
    shape.material = Material::new();
    shape.material.color = Color::new(1.0, 0.2, 1.0);

    //ch6 new: add a light source
    let light = Light::point_light(
        RayTuple::point(-10.0, 10.0, -10.0),
        Color::new(1.0, 1.0, 1.0),
    );

    let ray_origin = RayTuple::point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;

    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.0;

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * y as f64;
        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * x as f64;
            let position = RayTuple::point(world_x, world_y, wall_z);

            let r = Ray::new(ray_origin, (position - ray_origin).normalize());
            let xs = shape.intersect(r);

            let h = Intersection::hit(xs);
            match h {
                //ch6 new: we are now using the intersection to calculate the lighting at the hit
                Some(inter) => {
                    let point = r.position(inter.t);
                    let normal = inter.object.normal_at(point);
                    let eye = -r.direction;
                    let color = inter
                        .object
                        .material
                        .lighting(&light, point, eye, normal, false);
                    canvas.write_pixel(x, y, color);
                }
                None => continue,
            };
        }
    }
    canvas.save_ppm("chapter6sphere.ppm");
}
