#![allow(dead_code)]
use crate::camera::Camera;
use crate::color::Color;
use crate::computations::Computations;
use crate::intersection::Intersection;
use crate::light::Light;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::raytuple::RayTuple;
use crate::shape::{Shape, ShapeType};
use std::cmp::Ordering;
use std::f64::consts::FRAC_PI_3;

pub struct World {
    pub light: Light,
    pub objects: Vec<Shape>,
}

impl World {
    pub fn new() -> Self {
        Self {
            light: Light::point_light(
                RayTuple::point(-10.0, 10.0, -10.0),
                Color::new(1.0, 1.0, 1.0),
            ),
            objects: Vec::new(),
        }
    }

    pub fn default_world() -> Self {
        let mut s1 = Shape::new(ShapeType::Sphere);
        s1.material.color = Color::new(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;

        let mut s2 = Shape::new(ShapeType::Sphere);
        s2.transform = Matrix::scaling(0.5, 0.5, 0.5);

        Self {
            light: Light::point_light(
                RayTuple::point(-10.0, 10.0, -10.0),
                Color::new(1.0, 1.0, 1.0),
            ),
            objects: vec![s1, s2],
        }
    }

    pub fn intersect_world(&mut self, r: Ray) -> Vec<Intersection> {
        let mut resulting_intersections: Vec<Intersection> = Vec::new();

        for o in &mut self.objects {
            let mut xs = o.intersect(r);
            resulting_intersections.append(&mut xs);
        }
        resulting_intersections.sort_by(|a, b| {
            if a.t < b.t {
                Ordering::Less
            } else if a.t == b.t {
                Ordering::Equal
            } else {
                Ordering::Greater
            }
        });
        return resulting_intersections;
    }

    pub fn shade_hit(&mut self, comps: Computations, remaining: i32) -> Color {
        let shadowed = self.is_shadowed(comps.over_point);

        let surface = comps.object.material.lighting(
            comps.object,
            &self.light,
            comps.over_point,
            comps.eyev,
            comps.normalv,
            shadowed,
        );

        let reflected = self.reflected_color(comps, remaining);
        let refracted = self.refracted_color(comps, remaining);

        let material = comps.object.material;
        if material.reflective > 0.0 && material.transparency > 0.0 {
            let reflectance = Intersection::schlick(comps);
            return surface + reflected * reflectance + refracted * (1.0 - reflectance);
        }

        surface + reflected + refracted
    }

    pub fn color_at(&mut self, r: Ray, remaining: i32) -> Color {
        let xs = self.intersect_world(r);
        let option_hit = Intersection::hit(xs);
        if let Some(hit) = option_hit {
            let dummyxs: Vec<Intersection> = Vec::new(); //this is to fix refraction update
            let comps = hit.prepare_computations(r, dummyxs);
            self.shade_hit(comps, remaining)
        } else {
            return Color::new(0.0, 0.0, 0.0);
        }
    }

    pub fn is_shadowed(&mut self, p: RayTuple) -> bool {
        let v = self.light.position - p;
        let distance = v.magnitude();
        let direction = v.normalize();

        let r = Ray::new(p, direction);
        let intersections = self.intersect_world(r);

        if let Some(hit) = Intersection::hit(intersections) {
            if hit.t < distance {
                return true;
            }
        }

        false
    }

    pub fn reflected_color(&mut self, comps: Computations, remaining: i32) -> Color {
        if remaining < 1 || comps.object.material.reflective == 0.0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let reflect_ray = Ray::new(comps.over_point, comps.reflectv);
        let color = self.color_at(reflect_ray, remaining - 1);

        color * comps.object.material.reflective
    }

    pub fn refracted_color(&mut self, comps: Computations, remaining: i32) -> Color {
        let n_ratio = comps.n1 / comps.n2;
        let cos_i = comps.eyev.dot(comps.normalv);
        let sin2_t = n_ratio.powf(2.0) * (1.0 - cos_i.powf(2.0));

        if remaining == 0 || comps.object.material.transparency == 0.0 || sin2_t > 1.0 {
            Color::new(0.0, 0.0, 0.0)
        } else {
            let cos_t = (1.0_f64 - sin2_t).sqrt();
            let direction = comps.normalv * (n_ratio * cos_i - cos_t) - comps.eyev * n_ratio;
            let refract_ray = Ray::new(comps.under_point, direction);

            let color =
                self.color_at(refract_ray, remaining - 1) * comps.object.material.transparency;
            color
        }
    }

    pub fn chapter_twelve_cube() {
        let mut floor = Shape::plane();
        floor.material.color = Color::new(0.1, 0.1, 0.1);
        floor.material.specular = 0.0;
        floor.material.reflective = 0.3;

        let mut middle = Shape::glass_sphere();
        middle.transform = Matrix::translation(-0.5, 1.5, 0.5);
        middle.material.color = Color::new(0.05, 0.05, 0.1);

        let mut right = Shape::sphere();
        right.transform = Matrix::translation(-0.5, 1.5, 3.0) * Matrix::scaling(0.5, 0.5, 0.5);
        right.material.color = Color::new(1.0, 0.0, 0.0);
        right.material.diffuse = 0.7;
        right.material.specular = 0.3;

        let mut left = Shape::sphere();
        left.transform = Matrix::translation(-1.5, 0.33, -0.75) * Matrix::scaling(0.33, 0.33, 0.33);
        left.material.color = Color::new(0.1, 0.1, 0.9);
        left.material.diffuse = 0.7;
        left.material.specular = 0.3;

        let mut backright = Shape::sphere();
        backright.transform =
            Matrix::translation(1.0, 1.0, 2.0) * Matrix::scaling(0.75, 0.75, 0.75);
        backright.material.color = Color::new(1.0, 1.0, 1.0);
        backright.material.diffuse = 0.7;
        backright.material.specular = 0.3;
        backright.material.reflective = 0.5;
        backright.material.refractive_index = 1.1;

        let mut frontright = Shape::cube();
        frontright.transform =
            Matrix::translation(2.0, 0.5, 0.0) * Matrix::scaling(0.1,0.1,0.1) * Matrix::rotation_y(2.0_f64.sqrt() / 2.0);
        frontright.material.color = Color::new(0.3, 0.8, 0.2);
        frontright.material.diffuse = 0.7;
        frontright.material.specular = 0.3;
        frontright.material.reflective = 0.8;
        frontright.material.refractive_index = 1.5;

        let mut w = World::new();
        w.objects.push(floor);
        w.objects.push(middle);
        w.objects.push(left);
        w.objects.push(right);
        w.objects.push(backright);
        w.objects.push(frontright);

        //2560x1440p in 241s in release
        let mut c = Camera::new(2560, 1440, FRAC_PI_3);
        c.transform = Matrix::view_transform(
            RayTuple::point(0.0, 1.5, -5.0),
            RayTuple::point(0.0, 1.0, 0.0),
            RayTuple::vector(0.0, 1.0, 0.0),
        );

        let canvas = c.render(w);
        canvas.save_ppm("chapter12.ppm");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{intersections, pattern::Pattern};

    //I've done some modifications to this test, since we are using UUID's in sphere initialization,
    //I only test to make sure the objects contain the non-default characteristics.
    //Another option would to be to remove the uuid from the Sphere's PartialEQ implementation
    #[test]
    fn create_default_world() {
        let l = Light::point_light(
            RayTuple::point(-10.0, 10.0, -10.0),
            Color::new(1.0, 1.0, 1.0),
        );
        let mut s1 = Shape::new(ShapeType::Sphere);
        s1.material.color = Color::new(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;
        let mut s2 = Shape::new(ShapeType::Sphere);
        s2.transform = Matrix::scaling(0.5, 0.5, 0.5);
        let w = World::default_world();

        assert_eq!(w.light, l);
        assert_eq!(w.objects[0].material, s1.material);
        assert_eq!(w.objects[1].transform, s2.transform);
    }

    #[test]
    fn intersect_world_with_ray() {
        let mut w = World::default_world();
        let r = Ray::new(
            RayTuple::point(0.0, 0.0, -5.0),
            RayTuple::vector(0.0, 0.0, 1.0),
        );
        let xs = w.intersect_world(r);

        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 4.5);
        assert_eq!(xs[2].t, 5.5);
        assert_eq!(xs[3].t, 6.0);
    }

    #[test]
    fn shading_an_intersection() {
        let mut w = World::default_world();
        let r = Ray::new(
            RayTuple::point(0.0, 0.0, -5.0),
            RayTuple::vector(0.0, 0.0, 1.0),
        );
        let shape = w.objects[0];
        let i = Intersection::new(4.0, shape);
        let dummyxs: Vec<Intersection> = Vec::new(); //this is to fix refraction update
        let comps = i.prepare_computations(r, dummyxs);
        let c = w.shade_hit(comps, 5);

        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn shading_an_intersection_from_inside() {
        let mut w = World::default_world();
        w.light = Light::point_light(RayTuple::point(0.0, 0.25, 0.0), Color::new(1.0, 1.0, 1.0));

        let r = Ray::new(
            RayTuple::point(0.0, 0.0, 0.0),
            RayTuple::vector(0.0, 0.0, 1.0),
        );
        let shape = w.objects[1];
        let i = Intersection::new(0.5, shape);
        let dummyxs: Vec<Intersection> = Vec::new(); //this is to fix refraction update
        let comps = i.prepare_computations(r, dummyxs);
        let c = w.shade_hit(comps, 5);

        assert_eq!(c, Color::new(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn color_when_ray_misses() {
        let mut w = World::default_world();
        let r = Ray::new(
            RayTuple::point(0.0, 0.0, -5.0),
            RayTuple::vector(0.0, 1.0, 0.0),
        );
        let c = w.color_at(r, 5);
        assert_eq!(c, Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn color_when_ray_hits() {
        let mut w = World::default_world();
        let r = Ray::new(
            RayTuple::point(0.0, 0.0, -5.0),
            RayTuple::vector(0.0, 0.0, 1.0),
        );
        let c = w.color_at(r, 5);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn color_with_intersection_behind_ray() {
        let mut w = World::default_world();
        w.objects[0].material.ambient = 1.0;
        w.objects[1].material.ambient = 1.0;

        let r = Ray::new(
            RayTuple::point(0.0, 0.0, 0.75),
            RayTuple::vector(0.0, 0.0, -1.0),
        );
        let c = w.color_at(r, 5);
        assert_eq!(c, w.objects[1].material.color);
    }

    #[test]
    fn there_is_no_shadow() {
        let mut w = World::default_world();
        let p = RayTuple::point(0.0, 10.0, 0.0);

        assert!(!w.is_shadowed(p));
    }

    #[test]
    fn there_is_shadow() {
        let mut w = World::default_world();
        let p = RayTuple::point(10.0, -10.0, 10.0);

        assert!(w.is_shadowed(p));
    }

    #[test]
    fn there_is_no_shadow_object_behind_light() {
        let mut w = World::default_world();
        let p = RayTuple::point(-20.0, 20.0, -20.0);

        assert!(!w.is_shadowed(p));
    }

    #[test]
    fn there_is_no_shadow_point_infront() {
        let mut w = World::default_world();
        let p = RayTuple::point(-2.0, 2.0, -2.0);

        assert!(!w.is_shadowed(p));
    }

    #[test]
    fn shade_hit_is_given_intersection_in_shadow() {
        let mut w = World::new();
        w.light = Light::point_light(RayTuple::point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let s1 = Shape::new(ShapeType::Sphere);
        w.objects.push(s1);

        let mut s2 = Shape::new(ShapeType::Sphere);
        s2.transform = Matrix::translation(0.0, 0.0, 10.0);
        w.objects.push(s2);

        let r = Ray::new(
            RayTuple::point(0.0, 0.0, 5.0),
            RayTuple::vector(0.0, 0.0, 1.0),
        );
        let i = Intersection::new(4.0, s2);

        let dummyxs: Vec<Intersection> = Vec::new(); //this is to fix refraction update
        let comps = i.prepare_computations(r, dummyxs);
        let c = w.shade_hit(comps, 5);

        assert_eq!(c, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn reflected_color_for_nonreflective_material() {
        let mut w = World::default_world();
        let r = Ray::new(
            RayTuple::point(0.0, 0.0, 0.0),
            RayTuple::vector(0.0, 0.0, 1.0),
        );
        w.objects[1].material.ambient = 1.0;
        let i = Intersection::new(1.0, w.objects[1]);
        let dummyxs: Vec<Intersection> = Vec::new(); //this is to fix refraction update
        let comps = i.prepare_computations(r, dummyxs);
        let color = w.reflected_color(comps, 5);

        assert_eq!(color, Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn reflected_color_of_reflective_material() {
        let mut w = World::default_world();
        let mut shape = Shape::plane();
        shape.material.reflective = 0.5;
        shape.transform = Matrix::translation(0.0, -1.0, 0.0);
        w.objects.push(shape);
        let r = Ray::new(
            RayTuple::point(0.0, 0.0, -3.0),
            RayTuple::vector(0.0, -2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0),
        );
        let i = Intersection::new(2.0_f64.sqrt(), w.objects[2]);
        let dummyxs: Vec<Intersection> = Vec::new(); //this is to fix refraction update
        let comps = i.prepare_computations(r, dummyxs);
        let color = w.reflected_color(comps, 5);

        assert_eq!(color, Color::new(0.19033, 0.23791, 0.14274));
    }

    #[test]
    fn shade_hit_with_reflective_material() {
        let mut w = World::default_world();
        let mut shape = Shape::plane();
        shape.material.reflective = 0.5;
        shape.transform = Matrix::translation(0.0, -1.0, 0.0);
        w.objects.push(shape);
        let r = Ray::new(
            RayTuple::point(0.0, 0.0, -3.0),
            RayTuple::vector(0.0, -2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0),
        );
        let i = Intersection::new(2.0_f64.sqrt(), w.objects[2]);
        let dummyxs: Vec<Intersection> = Vec::new(); //this is to fix refraction update
        let comps = i.prepare_computations(r, dummyxs);
        let color = w.shade_hit(comps, 5);

        assert_eq!(color, Color::new(0.87675, 0.92434, 0.82917));
    }

    #[test]
    fn color_at_with_mutually_reflective_surfaces() {
        let mut w = World::new();
        w.light = Light::point_light(RayTuple::point(0.0, 0.0, 0.0), Color::new(1.0, 1.0, 1.0));

        let mut lower = Shape::plane();
        lower.material.reflective = 1.0;
        lower.transform = Matrix::translation(0.0, -1.0, 0.0);
        w.objects.push(lower);

        let mut upper = Shape::plane();
        upper.material.reflective = 1.0;
        upper.transform = Matrix::translation(0.0, 1.0, 0.0);
        w.objects.push(upper);

        let r = Ray::new(
            RayTuple::point(0.0, 0.0, 0.0),
            RayTuple::vector(0.0, 1.0, 0.0),
        );
        w.color_at(r, 5);
    }

    #[test]
    fn reflected_color_at_max_recursive_depth() {
        let mut w = World::default_world();

        let mut shape = Shape::plane();
        shape.material.reflective = 0.5;
        shape.transform = Matrix::translation(0.0, -1.0, 0.0);
        w.objects.push(shape);

        let r = Ray::new(
            RayTuple::point(0.0, 0.0, -3.0),
            RayTuple::vector(0.0, -2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0),
        );
        let i = Intersection::new(2.0_f64.sqrt(), shape);
        let dummyxs: Vec<Intersection> = Vec::new(); //this is to fix refraction update
        let comps = i.prepare_computations(r, dummyxs);
        let color = w.reflected_color(comps, 0);

        assert_eq!(color, Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn refracted_color_of_opaque_surface() {
        let mut w = World::default_world();
        let s = w.objects[0];
        let r = Ray::new(
            RayTuple::point(0.0, 0.0, -5.0),
            RayTuple::vector(0.0, 0.0, 1.0),
        );
        let xs = intersections!(Intersection::new(4.0, s), Intersection::new(6.0, s));
        let comps = xs[0].prepare_computations(r, xs);
        let c = w.refracted_color(comps, 5);

        assert_eq!(c, Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn refracted_color_at_maximum_recursive_depth() {
        let mut w = World::default_world();
        let s = &mut w.objects[0];
        s.material.transparency = 1.0;
        s.material.refractive_index = 1.5;
        let r = Ray::new(
            RayTuple::point(0.0, 0.0, -5.0),
            RayTuple::vector(0.0, 0.0, 1.0),
        );
        let xs = intersections!(Intersection::new(4.0, *s), Intersection::new(6.0, *s));
        let comps = xs[0].prepare_computations(r, xs);
        let c = w.refracted_color(comps, 0);

        assert_eq!(c, Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn refracted_color_under_total_internal_reflection() {
        let mut w = World::default_world();
        let s = &mut w.objects[0];
        s.material.transparency = 1.0;
        s.material.refractive_index = 1.5;

        let r = Ray::new(
            RayTuple::point(0.0, 0.0, 2.0_f64.sqrt() / 2.0),
            RayTuple::vector(0.0, 1.0, 0.0),
        );
        let xs = intersections!(
            Intersection::new(-2.0_f64.sqrt() / 2.0, *s),
            Intersection::new(2.0_f64.sqrt() / 2.0, *s)
        );

        let comps = xs[1].prepare_computations(r, xs);
        let c = w.refracted_color(comps, 5);

        assert_eq!(c, Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn refracted_color_with_refracted_ray() {
        let mut w = World::default_world();

        w.objects[0].material.ambient = 1.0;
        w.objects[0].material.pattern = Some(Pattern::test_pattern());

        w.objects[1].material.transparency = 1.0;
        w.objects[1].material.refractive_index = 1.5;

        let r = Ray::new(
            RayTuple::point(0.0, 0.0, 0.1),
            RayTuple::vector(0.0, 1.0, 0.0),
        );
        let xs = intersections!(
            Intersection::new(-0.9899, w.objects[0]),
            Intersection::new(-0.4899, w.objects[1]),
            Intersection::new(0.4899, w.objects[1]),
            Intersection::new(0.9899, w.objects[0])
        );
        let comps = xs[2].prepare_computations(r, xs);
        let c = w.refracted_color(comps, 5);

        //colors slightly adjusted for rounded book values
        assert_eq!(c, Color::new(0.0, 0.99887, 0.04721));
    }

    #[test]
    fn shade_hit_with_transparent_material() {
        let mut w = World::default_world();
        let mut floor = Shape::plane();
        floor.transform = Matrix::translation(0.0, -1.0, 0.0);
        floor.material.transparency = 0.5;
        floor.material.refractive_index = 1.5;
        w.objects.push(floor);

        let mut ball = Shape::sphere();
        ball.material.color = Color::new(1.0, 0.0, 0.0);
        ball.material.ambient = 0.5;
        ball.transform = Matrix::translation(0.0, -3.5, -0.5);
        w.objects.push(ball);

        let r = Ray::new(
            RayTuple::point(0.0, 0.0, -3.0),
            RayTuple::vector(0.0, -2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0),
        );
        let xs = intersections!(Intersection::new(2.0_f64.sqrt(), floor));

        let comps = xs[0].prepare_computations(r, xs);
        let color = w.shade_hit(comps, 5);

        assert_eq!(color, Color::new(0.93642, 0.68642, 0.68642));
    }

    #[test]
    fn shade_hit_with_reflective_transparent_material() {
        let mut w = World::default_world();
        let r = Ray::new(
            RayTuple::point(0.0, 0.0, -3.0),
            RayTuple::vector(0.0, -2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0),
        );

        let mut floor = Shape::plane();
        floor.transform = Matrix::translation(0.0, -1.0, 0.0);
        floor.material.reflective = 0.5;
        floor.material.transparency = 0.5;
        floor.material.refractive_index = 1.5;
        w.objects.push(floor);

        let mut ball = Shape::sphere();
        ball.material.color = Color::new(1.0, 0.0, 0.0);
        ball.material.ambient = 0.5;
        ball.transform = Matrix::translation(0.0, -3.5, -0.5);
        w.objects.push(ball);

        let xs = intersections!(Intersection::new(2.0_f64.sqrt(), floor));
        let comps = xs[0].prepare_computations(r, xs);
        let color = w.shade_hit(comps, 5);

        assert_eq!(color, Color::new(0.93391, 0.69643, 0.69243));
    }
}
