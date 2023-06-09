#![allow(dead_code)]
use crate::color::Color;
use crate::computations::Computations;
use crate::intersection::Intersection;
use crate::light::Light;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::raytuple::RayTuple;
use crate::shape::{Shape, ShapeType};
use std::cmp::Ordering;

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

    pub fn shade_hit(&mut self, comps: Computations) -> Color {
        let shadowed = self.is_shadowed(comps.over_point);

        comps.object.material.lighting(
            comps.object,
            &self.light,
            comps.over_point,
            comps.eyev,
            comps.normalv,
            shadowed,
        )
    }

    pub fn color_at(&mut self, r: Ray) -> Color {
        let xs = self.intersect_world(r);
        let option_hit = Intersection::hit(xs);
        if let Some(hit) = option_hit {
            let comps = hit.prepare_computations(r);
            self.shade_hit(comps)
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

        let h_option = Intersection::hit(intersections);
        match h_option {
            Some(h) => {
                if h.t < distance {
                    true
                } else {
                    false
                }
            }
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let comps = i.prepare_computations(r);
        let c = w.shade_hit(comps);

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
        let comps = i.prepare_computations(r);
        let c = w.shade_hit(comps);

        assert_eq!(c, Color::new(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn color_when_ray_misses() {
        let mut w = World::default_world();
        let r = Ray::new(
            RayTuple::point(0.0, 0.0, -5.0),
            RayTuple::vector(0.0, 1.0, 0.0),
        );
        let c = w.color_at(r);
        assert_eq!(c, Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn color_when_ray_hits() {
        let mut w = World::default_world();
        let r = Ray::new(
            RayTuple::point(0.0, 0.0, -5.0),
            RayTuple::vector(0.0, 0.0, 1.0),
        );
        let c = w.color_at(r);
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
        let c = w.color_at(r);
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

        let comps = i.prepare_computations(r);
        let c = w.shade_hit(comps);

        assert_eq!(c, Color::new(0.1, 0.1, 0.1));
    }
}
