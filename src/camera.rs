#![allow(dead_code)]
use std::f64::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_4};

use crate::canvas::Canvas;
use crate::color::Color;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::raytuple::RayTuple;
use crate::sphere::Sphere;
use crate::world::World;

#[derive(Debug)]
pub struct Camera {
    hsize: i32,
    vsize: i32,
    field_of_view: f64,
    transform: Matrix,
    pixel_size: f64,
    half_width: f64,
    half_height: f64,
}

impl Camera {
    pub fn new(hsize: i32, vsize: i32, field_of_view: f64) -> Self {
        let half_view = (field_of_view / 2.0).tan();
        let aspect = hsize as f64 / vsize as f64;
        let half_width = match aspect >= 1.0 {
            true => half_view,
            false => half_view * aspect,
        };
        let half_height = match aspect >= 1.0 {
            true => half_view / aspect,
            false => half_view,
        };
        Self {
            hsize,
            vsize,
            field_of_view,
            transform: Matrix::identity(),
            pixel_size: (half_width * 2.0) / hsize as f64,
            half_width,
            half_height,
        }
    }

    pub fn ray_for_pixel(&self, px: i32, py: i32) -> Ray {
        let xoffset = (px as f64 + 0.5) * self.pixel_size;
        let yoffset = (py as f64 + 0.5) * self.pixel_size;

        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        let pixel = self.transform.inverse().unwrap() * RayTuple::point(world_x, world_y, -1.0);
        let origin = self.transform.inverse().unwrap() * RayTuple::point(0.0, 0.0, 0.0);
        let direction = (pixel - origin).normalize();

        Ray::new(origin, direction)
    }

    pub fn render(self, w: World) -> Canvas {
        let mut image = Canvas::new(self.hsize, self.vsize);

        let camera_vsize = self.vsize;
        let camera_hsize = self.hsize;
        for y in 0..camera_vsize {
            for x in 0..camera_hsize {
                let ray = self.ray_for_pixel(x, y);
                let color = w.color_at(ray);
                image.write_pixel(x, y, color);
            }
        }
        image
    }
}

pub fn chapter_seven_scene() {
    let mut floor = Sphere::new();
    floor.set_transform(Matrix::scaling(10.0, 0.01, 10.0));
    floor.material.color = Color::new(1.0, 0.9, 0.9);
    floor.material.specular = 0.0;

    let mut left_wall = Sphere::new();
    left_wall.set_transform(
        Matrix::translation(0.0, 0.0, 5.0)
            * Matrix::rotation_y(-FRAC_PI_4)
            * Matrix::rotation_x(FRAC_PI_2)
            * Matrix::scaling(10.0, 0.01, 10.0),
    );
    left_wall.material = floor.material;

    let mut right_wall = Sphere::new();
    right_wall.set_transform(
        Matrix::translation(0.0, 0.0, 5.0)
            * Matrix::rotation_y(FRAC_PI_4)
            * Matrix::rotation_x(FRAC_PI_2)
            * Matrix::scaling(10.0, 0.01, 10.0),
    );
    right_wall.material = floor.material;

    let mut middle = Sphere::new();
    middle.set_transform(Matrix::translation(-0.5, 1.0, 0.5));
    middle.material.color = Color::new(0.1, 1.0, 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;

    let mut right = Sphere::new();
    right.set_transform(Matrix::translation(1.5, 0.5, -0.5) * Matrix::scaling(0.5, 0.5, 0.5));
    right.material.color = Color::new(0.5, 1.0, 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;

    let mut left = Sphere::new();
    left.set_transform(Matrix::translation(-1.5, 0.33, -0.75) * Matrix::scaling(0.33, 0.33, 0.33));
    left.material.color = Color::new(1.0, 0.8, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;

    let mut w = World::new();
    w.objects.push(floor);
    w.objects.push(left_wall);
    w.objects.push(right_wall);
    w.objects.push(middle);
    w.objects.push(left);
    w.objects.push(right);

    //800x600 renders in release mode in just under 18 seconds
    //change the camera from view_transform to a higher y and a much further back negative z (-20.0)\
    //to see the stretched floor/wall spheres in action
    let mut c = Camera::new(800, 600, FRAC_PI_3);
    c.transform = Matrix::view_transform(
        RayTuple::point(0.0, 1.5, -5.0), 
        RayTuple::point(0.0, 1.0, 0.0),
        RayTuple::vector(0.0, 1.0, 0.0),
    );

    let canvas = c.render(w);
    canvas.save_ppm("chapter7.ppm");
}

#[cfg(test)]
mod tests {
    use crate::raytuple::RayTuple;

    use super::*;
    use crate::color::Color;
    use std::f64::consts::{FRAC_PI_2, FRAC_PI_4};

    #[test]
    fn create_a_camera() {
        let hsize = 160;
        let vsize = 120;
        let fov = FRAC_PI_2;

        let c = Camera::new(hsize, vsize, fov);
        assert_eq!(c.hsize, 160);
        assert_eq!(c.vsize, 120);
        assert_eq!(c.field_of_view, FRAC_PI_2);
        assert_eq!(c.transform, Matrix::identity());
    }

    #[test]
    fn pixel_size_horizontal() {
        let c = Camera::new(200, 125, FRAC_PI_2);
        assert!((c.pixel_size - 0.01).abs() < f64::EPSILON);
    }

    #[test]
    fn pixel_size_vertical() {
        let c = Camera::new(125, 200, FRAC_PI_2);
        assert!((c.pixel_size - 0.01).abs() < f64::EPSILON);
    }

    #[test]
    fn ray_through_center() {
        let c = Camera::new(201, 101, FRAC_PI_2);
        let r = c.ray_for_pixel(100, 50);

        assert_eq!(r.origin, RayTuple::point(0.0, 0.0, 0.0));
        assert_eq!(r.direction, RayTuple::vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn ray_through_corner() {
        let c = Camera::new(201, 101, FRAC_PI_2);
        let r = c.ray_for_pixel(0, 0);

        assert_eq!(r.origin, RayTuple::point(0.0, 0.0, 0.0));
        assert_eq!(r.direction, RayTuple::vector(0.66519, 0.33259, -0.66851));
    }

    #[test]
    fn ray_with_transformed_camera() {
        let mut c = Camera::new(201, 101, FRAC_PI_2);
        c.transform = Matrix::rotation_y(FRAC_PI_4) * Matrix::translation(0.0, -2.0, 5.0);
        let r = c.ray_for_pixel(100, 50);

        assert_eq!(r.origin, RayTuple::point(0.0, 2.0, -5.0));
        assert_eq!(
            r.direction,
            RayTuple::vector(2.0_f64.sqrt() / 2.0, 0.0, -2.0_f64.sqrt() / 2.0)
        );
    }

    #[test]
    fn render_world_with_camera() {
        let w = World::default_world();
        let mut c = Camera::new(11, 11, FRAC_PI_2);
        let from = RayTuple::point(0.0, 0.0, -5.0);
        let to = RayTuple::point(0.0, 0.0, 0.0);
        let up = RayTuple::vector(0.0, 1.0, 0.0);
        c.transform = Matrix::view_transform(from, to, up);
        let image = c.render(w);

        assert_eq!(image.pixel_at(5, 5), Color::new(0.38066, 0.47583, 0.2855));
    }
}
