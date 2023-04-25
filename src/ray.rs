#![allow(dead_code)]
use crate::intersection::Intersection;
use crate::raytuple::RayTuple;
use crate::sphere::Sphere;

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

    pub fn intersects(self, _sphere: Sphere) -> Vec<f64> {
        let mut intersection_times: Vec<f64> = Vec::new();

        let sphere_to_ray = self.origin - RayTuple::point(0.0, 0.0, 0.0);
        let a = self.direction.dot(self.direction);
        let b = 2.0 * self.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;

        let discriminant = b.powf(2.0) - 4.0 * a * c;
        if discriminant < 0.0 {
            return intersection_times;
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
        intersection_times.push(t1);
        intersection_times.push(t2);

        intersection_times
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
    fn ray_intersects_sphere_at_two_points() {
        let r = Ray::new(
            RayTuple::point(0.0, 0.0, -5.0),
            RayTuple::vector(0.0, 0.0, 1.0),
        );
        let s = Sphere::new();
        let xs = r.intersects(s);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], 4.0);
        assert_eq!(xs[1], 6.0);
    }

    #[test]
    fn ray_intersects_sphere_at_tangent() {
        let r = Ray::new(
            RayTuple::point(0.0, 1.0, -5.0),
            RayTuple::vector(0.0, 0.0, 1.0),
        );
        let s = Sphere::new();
        let xs = r.intersects(s);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], 5.0);
        assert_eq!(xs[1], 5.0);
    }

    #[test]
    fn ray_misses_sphere() {
        let r = Ray::new(
            RayTuple::point(0.0, 2.0, -5.0),
            RayTuple::vector(0.0, 0.0, 1.0),
        );
        let s = Sphere::new();
        let xs = r.intersects(s);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let r = Ray::new(
            RayTuple::point(0.0, 0.0, 0.0),
            RayTuple::vector(0.0, 0.0, 1.0),
        );
        let s = Sphere::new();
        let xs = r.intersects(s);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], -1.0);
        assert_eq!(xs[1], 1.0);
    }

    #[test]
    fn sphere_is_behind_ray() {
        let r = Ray::new(
            RayTuple::point(0.0, 0.0, 5.0),
            RayTuple::vector(0.0, 0.0, 1.0),
        );
        let s = Sphere::new();
        let xs = r.intersects(s);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], -6.0);
        assert_eq!(xs[1], -4.0);
    }
}
