#![allow(dead_code)]
use crate::intersection::Intersection;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::raytuple::RayTuple;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ShapeType {
    Sphere,
    Plane,
    Test,
}

#[derive(Debug, Clone, Copy)]
pub struct Shape {
    id: Uuid,
    shape_type: ShapeType,
    pub transform: Matrix,
    pub material: Material,
    pub saved_ray: Ray,
}

impl Shape {
    pub fn new(shape_type: ShapeType) -> Self {
        Self {
            id: Uuid::new_v4(),
            shape_type,
            transform: Matrix::identity(),
            material: Material::new(),
            saved_ray: Ray::new( RayTuple::point(3.0, 0.0, 0.0), RayTuple::vector(3.0, 0.0, 0.0) ),
        }
    }

    pub fn test_shape() -> Self {
        Self {
            id: Uuid::new_v4(),
            shape_type: ShapeType::Test,
            transform: Matrix::identity(),
            material: Material::new(),
            saved_ray: Ray::new( RayTuple::point(3.0, 0.0, 0.0), RayTuple::vector(3.0, 0.0, 0.0) ),
        }
    }

    pub fn id(self) -> Uuid {
        self.id
    }

    pub fn intersect(&mut self, r: Ray) -> Vec<Intersection> {
        let mut intersections: Vec<Intersection> = Vec::new();
        let local_inverse_transform = self.transform.inverse();
        if let None = local_inverse_transform {
            return intersections;
        }
        self.saved_ray = r.transform(local_inverse_transform.unwrap());

        match self.shape_type {
            ShapeType::Sphere => {
                let sphere_to_ray = self.saved_ray.origin - RayTuple::point(0.0, 0.0, 0.0);
                let a = self.saved_ray.direction.dot(self.saved_ray.direction);
                let b = 2.0 * self.saved_ray.direction.dot(sphere_to_ray);
                let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;

                let discriminant = b.powf(2.0) - 4.0 * a * c;
                if discriminant < 0.0 {
                    return intersections;
                }

                let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
                let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
                intersections.push(Intersection::new(t1, *self));
                intersections.push(Intersection::new(t2, *self));

                intersections
            }
            ShapeType::Plane => Vec::new(),
            ShapeType::Test => Vec::new(),
        }
    }

    pub fn normal_at(self, world_point: RayTuple) -> RayTuple {
        match self.shape_type {
            ShapeType::Sphere => {
                let object_point = self.transform.inverse().unwrap() * world_point;
                let object_normal = object_point - RayTuple::point(0.0, 0.0, 0.0);
                let mut world_normal =
                    self.transform.inverse().unwrap().transpose() * object_normal;
                world_normal.w = 0.0;

                world_normal.normalize()
            }
            ShapeType::Plane => RayTuple::vector(0.0, 1.0, 0.0),
            ShapeType::Test => unreachable!("Call to normal_at on Test Shape"),
        }
    }
}

impl PartialEq for Shape {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.shape_type == other.shape_type
            && self.transform == other.transform
            && self.material == other.material
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
        let mut s = Shape::new(ShapeType::Sphere);
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
        let mut s = Shape::new(ShapeType::Sphere);
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
        let mut s = Shape::new(ShapeType::Sphere);
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let r = Ray::new(
            RayTuple::point(0.0, 0.0, 0.0),
            RayTuple::vector(0.0, 0.0, 1.0),
        );
        let mut s = Shape::new(ShapeType::Sphere);
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
        let mut s = Shape::new(ShapeType::Sphere);
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
        let mut s: Shape = Shape::new(ShapeType::Sphere);
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].object, s);
        assert_eq!(xs[1].object, s);
    }

    #[test]
    fn sphere_default_transform() {
        let s = Shape::new(ShapeType::Sphere);

        assert_eq!(s.transform, Matrix::identity());
    }

    #[test]
    fn sphere_set_transform() {
        let mut s = Shape::new(ShapeType::Sphere);
        let t = Matrix::translation(2.0, 3.0, 4.0);
        s.transform = t;

        assert_eq!(s.transform, t);
    }

    #[test]
    fn intersect_scaled_sphere() {
        let r = Ray::new(
            RayTuple::point(0.0, 0.0, -5.0),
            RayTuple::vector(0.0, 0.0, 1.0),
        );
        let mut s = Shape::new(ShapeType::Sphere);
        s.transform = Matrix::scaling(2.0, 2.0, 2.0);
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
        let mut s = Shape::new(ShapeType::Sphere);
        s.transform = Matrix::translation(5.0, 0.0, 0.0);
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn normal_spere_at_x_axis() {
        let s = Shape::new(ShapeType::Sphere);
        let n = s.normal_at(RayTuple::point(1.0, 0.0, 0.0));

        assert_eq!(n, RayTuple::vector(1.0, 0.0, 0.0));
    }

    #[test]
    fn normal_sphere_at_y_axis() {
        let s = Shape::new(ShapeType::Sphere);
        let n = s.normal_at(RayTuple::point(0.0, 1.0, 0.0));

        assert_eq!(n, RayTuple::vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn normal_sphere_at_z_axis() {
        let s = Shape::new(ShapeType::Sphere);
        let n = s.normal_at(RayTuple::point(0.0, 0.0, 1.0));

        assert_eq!(n, RayTuple::vector(0.0, 0.0, 1.0));
    }

    #[test]
    fn normal_non_axial() {
        let coord: f64 = 3.0_f64.sqrt() / 3.0;
        let s = Shape::new(ShapeType::Sphere);
        let n = s.normal_at(RayTuple::point(coord, coord, coord));

        assert_eq!(n, RayTuple::vector(coord, coord, coord));
    }

    #[test]
    fn normal_on_translated_sphere() {
        let mut s = Shape::new(ShapeType::Sphere);
        s.transform = Matrix::translation(0.0, 1.0, 0.0);
        let n = s.normal_at(RayTuple::point(0.0, 1.70711, -0.70711));

        assert_eq!(n, RayTuple::vector(0.0, 0.70711, -0.70711));
    }

    #[test]
    fn normal_on_transformed_sphere() {
        let mut s = Shape::new(ShapeType::Sphere);
        let m = Matrix::scaling(1.0, 0.5, 1.0) * Matrix::rotation_z(PI / 5.0);
        s.transform = m;
        let n = s.normal_at(RayTuple::point(
            0.0,
            2.0_f64.sqrt() / 2.0,
            -2.0_f64.sqrt() / 2.0,
        ));

        assert_eq!(n, RayTuple::vector(0.0, 0.97014, -0.24254));
    }

    #[test]
    fn sphere_has_default_material() {
        let s = Shape::new(ShapeType::Sphere);

        assert_eq!(s.material, Material::new());
    }

    #[test]
    fn sphere_material_can_be_set() {
        let mut s = Shape::new(ShapeType::Sphere);
        let mut m = Material::new();
        m.ambient = 1.0;
        s.material = m;
        assert_eq!(s.material, m);
    }

    #[test]
    fn test_shape_has_default_transform() {
        let s = Shape::test_shape();
        assert_eq!(s.transform, Matrix::identity());
    }

    #[test]
    fn test_shape_assign_transform() {
        let mut s = Shape::test_shape();
        s.transform = Matrix::translation(2.0, 3.0, 4.0);
        assert_eq!(s.transform, Matrix::translation(2.0, 3.0, 4.0));
    }

    #[test]
    fn test_has_default_material() {
        let s = Shape::test_shape();
        let m = s.material;
        assert_eq!(m, Material::new());
    }

    #[test]
    fn test_can_assign_material() {
        let mut s = Shape::test_shape();
        let mut m = Material::new();
        m.ambient = 1.0;
        s.material = m;
        assert_eq!(s.material, m);
    }

    #[test]
    fn intersecting_scaled_shape_with_ray() {
        let r = Ray::new(RayTuple::point(0.0, 0.0, -5.0), RayTuple::vector(0.0,0.0,1.0));
        let mut s = Shape::test_shape();
        s.transform = Matrix::scaling(2.0, 2.0, 2.0);
        let _xs = s.intersect(r);

        assert_eq!(s.saved_ray.origin, RayTuple::point(0.0, 0.0, -2.5));
        assert_eq!(s.saved_ray.direction, RayTuple::vector(0.0, 0.0, 0.5));
    }

    #[test]
    fn intersecting_translated_shape_with_ray() {
        let r = Ray::new(RayTuple::point(0.0, 0.0, -5.0), RayTuple::vector(0.0,0.0,1.0));
        let mut s = Shape::test_shape();
        s.transform = Matrix::translation(5.0, 0.0, 0.0);
        let _xs = s.intersect(r);

        assert_eq!(s.saved_ray.origin, RayTuple::point(-5.0, 0.0, -5.0));
        assert_eq!(s.saved_ray.direction, RayTuple::vector(0.0, 0.0, 1.0));
    }
}
