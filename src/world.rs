#![allow(dead_code)]
use crate::color::Color;
use crate::computations::Computations;
use crate::intersection::Intersection;
use crate::light::Light;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::raytuple::RayTuple;
use crate::sphere::Sphere;
use std::cmp::Ordering;

pub struct World {
    light: Light,
    objects: Vec<Sphere>,
}

impl World {
    //here we are skipping the book's empty world creation because it's a hassle to
    //initiate a struct without giving it's members values.  We can always refactor
    //and put the light in a Vec so that it can be implemented without any lights
    //in the Vec if we need to, but I don't see that so far.

    pub fn default_world() -> Self {
        let mut s1 = Sphere::new();
        s1.material.color = Color::new(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;

        let mut s2 = Sphere::new();
        s2.set_transform(Matrix::scaling(0.5, 0.5, 0.5));

        Self {
            light: Light::point_light(
                RayTuple::point(-10.0, 10.0, -10.0),
                Color::new(1.0, 1.0, 1.0),
            ),
            objects: vec![s1, s2],
        }
    }

    pub fn intersect_world(&self, r: Ray) -> Vec<Intersection> {
        let mut resulting_intersections: Vec<Intersection> = Vec::new();

        for o in &self.objects {
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

    pub fn shade_hit(&self, comps: Computations) -> Color {
        comps
            .object
            .material
            .lighting(&self.light, comps.point, comps.eyev, comps.normalv)
    }

    pub fn color_at(&self, r: Ray) -> Color {
        let xs = self.intersect_world(r);
        let option_hit = Intersection::hit(xs);
        if let Some(hit) = option_hit {
            let comps = hit.prepare_computations(r);
            self.shade_hit(comps)
        } else {
            return Color::new(0.0, 0.0, 0.0);
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
        let mut s1 = Sphere::new();
        s1.material.color = Color::new(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;
        let mut s2 = Sphere::new();
        s2.set_transform(Matrix::scaling(0.5, 0.5, 0.5));
        let w = World::default_world();

        assert_eq!(w.light, l);
        assert_eq!(w.objects[0].material, s1.material);
        assert_eq!(w.objects[1].get_transform(), s2.get_transform());
    }

    #[test]
    fn intersect_world_with_ray() {
        let w = World::default_world();
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
        let w = World::default_world();
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
        let w = World::default_world();
        let r = Ray::new(
            RayTuple::point(0.0, 0.0, -5.0),
            RayTuple::vector(0.0, 1.0, 0.0),
        );
        let c = w.color_at(r);
        assert_eq!(c, Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn color_when_ray_hits() {
        let w = World::default_world();
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
}
