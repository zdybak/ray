#![allow(dead_code)]
use crate::intersection::Intersection;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::raytuple::RayTuple;
use uuid::Uuid;

//We have to clone/copy sphere objects to store the same object in multiple intersections
#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    id: Uuid,
    transform: Matrix,
    pub material: Material,
}

impl Sphere {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            transform: Matrix::identity(),
            material: Material::new(),
        }
    }

    pub fn id(self) -> Uuid {
        self.id
    }

    pub fn set_transform(&mut self, m: Matrix) {
        self.transform = m;
    }

    pub fn get_transform(self) -> Matrix {
        self.transform
    }

    pub fn intersect(self, r: Ray) -> Vec<Intersection> {
        let mut intersections: Vec<Intersection> = Vec::new();

        let sphere_inverse_transform = self.transform.inverse();
        if let None = sphere_inverse_transform {
            return intersections;
        }
        let r = r.transform(sphere_inverse_transform.unwrap());

        let sphere_to_ray = r.origin - RayTuple::point(0.0, 0.0, 0.0);
        let a = r.direction.dot(r.direction);
        let b = 2.0 * r.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;

        let discriminant = b.powf(2.0) - 4.0 * a * c;
        if discriminant < 0.0 {
            return intersections;
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
        intersections.push(Intersection::new(t1, self));
        intersections.push(Intersection::new(t2, self));

        intersections
    }

    pub fn normal_at(self, world_point: RayTuple) -> RayTuple {
        let object_point = self.transform.inverse().unwrap() * world_point;
        let object_normal = object_point - RayTuple::point(0.0, 0.0, 0.0);
        let mut world_normal = self.transform.inverse().unwrap().transpose() * object_normal;
        world_normal.w = 0.0;

        world_normal.normalize()
    }
}

impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.transform == other.transform && self.material == other.material
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn ray_intersects_sphere_at_two_points() {
        let r = Ray::new(
            RayTuple::point(0.0, 0.0, -5.0),
            RayTuple::vector(0.0, 0.0, 1.0),
        );
        let s = Sphere::new();
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);
    }

    #[test]
    fn ray_intersects_sphere_at_tangent() {
        let r = Ray::new(
            RayTuple::point(0.0, 1.0, -5.0),
            RayTuple::vector(0.0, 0.0, 1.0),
        );
        let s = Sphere::new();
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 5.0);
        assert_eq!(xs[1].t, 5.0);
    }

    #[test]
    fn ray_misses_sphere() {
        let r = Ray::new(
            RayTuple::point(0.0, 2.0, -5.0),
            RayTuple::vector(0.0, 0.0, 1.0),
        );
        let s = Sphere::new();
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let r = Ray::new(
            RayTuple::point(0.0, 0.0, 0.0),
            RayTuple::vector(0.0, 0.0, 1.0),
        );
        let s = Sphere::new();
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[1].t, 1.0);
    }

    #[test]
    fn sphere_is_behind_ray() {
        let r = Ray::new(
            RayTuple::point(0.0, 0.0, 5.0),
            RayTuple::vector(0.0, 0.0, 1.0),
        );
        let s = Sphere::new();
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -6.0);
        assert_eq!(xs[1].t, -4.0);
    }

    #[test]
    fn intersect_returns_object() {
        let r = Ray::new(
            RayTuple::point(0.0, 0.0, -5.0),
            RayTuple::vector(0.0, 0.0, 1.0),
        );
        let s = Sphere::new();
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].object, s);
        assert_eq!(xs[1].object, s);
    }

    #[test]
    fn sphere_default_transform() {
        let s = Sphere::new();

        assert_eq!(s.transform, Matrix::identity());
    }

    #[test]
    fn sphere_set_transform() {
        let mut s = Sphere::new();
        let t = Matrix::translation(2.0, 3.0, 4.0);
        s.set_transform(t);

        assert_eq!(s.transform, t);
    }

    #[test]
    fn intersect_scaled_sphere() {
        let r = Ray::new(
            RayTuple::point(0.0, 0.0, -5.0),
            RayTuple::vector(0.0, 0.0, 1.0),
        );
        let mut s = Sphere::new();
        s.set_transform(Matrix::scaling(2.0, 2.0, 2.0));
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 3.0);
        assert_eq!(xs[1].t, 7.0);
    }

    #[test]
    fn intersect_translated_sphere() {
        let r = Ray::new(
            RayTuple::point(0.0, 0.0, -5.0),
            RayTuple::vector(0.0, 0.0, 1.0),
        );
        let mut s = Sphere::new();
        s.set_transform(Matrix::translation(5.0, 0.0, 0.0));
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn normal_spere_at_x_axis() {
        let s = Sphere::new();
        let n = s.normal_at(RayTuple::point(1.0, 0.0, 0.0));

        assert_eq!(n, RayTuple::vector(1.0, 0.0, 0.0));
    }

    #[test]
    fn normal_sphere_at_y_axis() {
        let s = Sphere::new();
        let n = s.normal_at(RayTuple::point(0.0, 1.0, 0.0));

        assert_eq!(n, RayTuple::vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn normal_sphere_at_z_axis() {
        let s = Sphere::new();
        let n = s.normal_at(RayTuple::point(0.0, 0.0, 1.0));

        assert_eq!(n, RayTuple::vector(0.0, 0.0, 1.0));
    }

    #[test]
    fn normal_non_axial() {
        let coord: f64 = 3.0_f64.sqrt() / 3.0;
        let s = Sphere::new();
        let n = s.normal_at(RayTuple::point(coord, coord, coord));

        assert_eq!(n, RayTuple::vector(coord, coord, coord));
    }

    #[test]
    fn normal_on_translated_sphere() {
        let mut s = Sphere::new();
        s.set_transform(Matrix::translation(0.0, 1.0, 0.0));
        let n = s.normal_at(RayTuple::point(0.0, 1.70711, -0.70711));

        assert_eq!(n, RayTuple::vector(0.0, 0.70711, -0.70711));
    }

    #[test]
    fn normal_on_transformed_sphere() {
        let mut s = Sphere::new();
        let m = Matrix::scaling(1.0, 0.5, 1.0) * Matrix::rotation_z(PI / 5.0);
        s.set_transform(m);
        let n = s.normal_at(RayTuple::point(
            0.0,
            2.0_f64.sqrt() / 2.0,
            -2.0_f64.sqrt() / 2.0,
        ));

        assert_eq!(n, RayTuple::vector(0.0, 0.97014, -0.24254));
    }

    #[test]
    fn sphere_has_default_material() {
        let s = Sphere::new();

        assert_eq!(s.material, Material::new());
    }

    #[test]
    fn sphere_material_can_be_set() {
        let mut s = Sphere::new();
        let mut m = Material::new();
        m.ambient = 1.0;
        s.material = m;
        assert_eq!(s.material, m);
    }
}
