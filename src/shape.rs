#![allow(dead_code)]
use crate::camera::Camera;
use crate::color::Color;
use crate::intersection::Intersection;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::raytuple::RayTuple;
use crate::world::World;
use std::f64::consts::FRAC_PI_3;
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
            saved_ray: Ray::new(
                RayTuple::point(0.0, 0.0, 0.0),
                RayTuple::vector(0.0, 0.0, 0.0),
            ),
        }
    }

    pub fn test_shape() -> Self {
        Self {
            id: Uuid::new_v4(),
            shape_type: ShapeType::Test,
            transform: Matrix::identity(),
            material: Material::new(),
            saved_ray: Ray::new(
                RayTuple::point(0.0, 0.0, 0.0),
                RayTuple::vector(0.0, 0.0, 0.0),
            ),
        }
    }

    pub fn sphere() -> Self {
        Self {
            id: Uuid::new_v4(),
            shape_type: ShapeType::Sphere,
            transform: Matrix::identity(),
            material: Material::new(),
            saved_ray: Ray::new(
                RayTuple::point(0.0, 0.0, 0.0),
                RayTuple::vector(0.0, 0.0, 0.0),
            ),
        }
    }

    pub fn plane() -> Self {
        Self {
            id: Uuid::new_v4(),
            shape_type: ShapeType::Plane,
            transform: Matrix::identity(),
            material: Material::new(),
            saved_ray: Ray::new(
                RayTuple::point(0.0, 0.0, 0.0),
                RayTuple::vector(0.0, 0.0, 0.0),
            ),
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
            ShapeType::Plane => {
                let epsilon: f64 = 0.00001;
                if r.direction.y.abs() < epsilon {
                    return intersections;
                }

                let t = -r.origin.y / r.direction.y;

                intersections.push(Intersection::new(t, *self));
                intersections
            }
            ShapeType::Test => intersections,
        }
    }

    pub fn normal_at(self, world_point: RayTuple) -> RayTuple {
        let object_point = self.transform.inverse().unwrap() * world_point;

        match self.shape_type {
            ShapeType::Sphere => {
                let object_normal = object_point - RayTuple::point(0.0, 0.0, 0.0);
                let mut world_normal =
                    self.transform.inverse().unwrap().transpose() * object_normal;
                world_normal.w = 0.0;

                world_normal.normalize()
            }
            ShapeType::Plane => RayTuple::vector(0.0, 1.0, 0.0),
            ShapeType::Test => {
                let local_normal = RayTuple::vector(object_point.x, object_point.y, object_point.z);
                let mut world_normal = self.transform.inverse().unwrap().transpose() * local_normal;
                world_normal.w = 0.0;

                world_normal.normalize()
            }
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

pub fn chapter_nine_plane() {
    let mut floor = Shape::plane();
    floor.material.color = Color::new(1.0, 0.9, 0.9);
    floor.material.specular = 0.0;

    let mut middle = Shape::sphere();
    middle.transform = Matrix::translation(-0.5, 1.0, 0.5);
    middle.material.color = Color::new(0.1, 1.0, 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;

    let mut right = Shape::sphere();
    right.transform = Matrix::translation(1.5, 0.5, -0.5) * Matrix::scaling(0.5, 0.5, 0.5);
    right.material.color = Color::new(0.5, 1.0, 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;

    let mut left = Shape::sphere();
    left.transform = Matrix::translation(-1.5, 0.33, -0.75) * Matrix::scaling(0.33, 0.33, 0.33);
    left.material.color = Color::new(1.0, 0.8, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;

    let mut w = World::new();
    w.objects.push(floor);
    w.objects.push(middle);
    w.objects.push(left);
    w.objects.push(right);

    //800x600 after shadows in debug takes 400s
    //256x256 after shadows in release takes less than 5 seconds
    //2560x1440p in 235s in release
    let mut c = Camera::new(2560, 1440, FRAC_PI_3);
    c.transform = Matrix::view_transform(
        RayTuple::point(0.0, 1.5, -5.0),
        RayTuple::point(0.0, 1.0, 0.0),
        RayTuple::vector(0.0, 1.0, 0.0),
    );

    let canvas = c.render(w);
    canvas.save_ppm("chapter9.ppm");
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
        let mut s = Shape::sphere();
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
        let mut s = Shape::sphere();
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
        let mut s = Shape::sphere();
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let r = Ray::new(
            RayTuple::point(0.0, 0.0, 0.0),
            RayTuple::vector(0.0, 0.0, 1.0),
        );
        let mut s = Shape::sphere();
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
        let mut s = Shape::sphere();
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
        let mut s: Shape = Shape::sphere();
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].object, s);
        assert_eq!(xs[1].object, s);
    }

    #[test]
    fn sphere_default_transform() {
        let s = Shape::sphere();

        assert_eq!(s.transform, Matrix::identity());
    }

    #[test]
    fn sphere_set_transform() {
        let mut s = Shape::sphere();
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
        let mut s = Shape::sphere();
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
        let mut s = Shape::sphere();
        s.transform = Matrix::translation(5.0, 0.0, 0.0);
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn normal_spere_at_x_axis() {
        let s = Shape::sphere();
        let n = s.normal_at(RayTuple::point(1.0, 0.0, 0.0));

        assert_eq!(n, RayTuple::vector(1.0, 0.0, 0.0));
    }

    #[test]
    fn normal_sphere_at_y_axis() {
        let s = Shape::sphere();
        let n = s.normal_at(RayTuple::point(0.0, 1.0, 0.0));

        assert_eq!(n, RayTuple::vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn normal_sphere_at_z_axis() {
        let s = Shape::sphere();
        let n = s.normal_at(RayTuple::point(0.0, 0.0, 1.0));

        assert_eq!(n, RayTuple::vector(0.0, 0.0, 1.0));
    }

    #[test]
    fn normal_non_axial() {
        let coord: f64 = 3.0_f64.sqrt() / 3.0;
        let s = Shape::sphere();
        let n = s.normal_at(RayTuple::point(coord, coord, coord));

        assert_eq!(n, RayTuple::vector(coord, coord, coord));
    }

    #[test]
    fn normal_on_translated_sphere() {
        let mut s = Shape::sphere();
        s.transform = Matrix::translation(0.0, 1.0, 0.0);
        let n = s.normal_at(RayTuple::point(0.0, 1.70711, -0.70711));

        assert_eq!(n, RayTuple::vector(0.0, 0.70711, -0.70711));
    }

    #[test]
    fn normal_on_transformed_sphere() {
        let mut s = Shape::sphere();
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
        let s = Shape::sphere();

        assert_eq!(s.material, Material::new());
    }

    #[test]
    fn sphere_material_can_be_set() {
        let mut s = Shape::sphere();
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
        let r = Ray::new(
            RayTuple::point(0.0, 0.0, -5.0),
            RayTuple::vector(0.0, 0.0, 1.0),
        );
        let mut s = Shape::test_shape();
        s.transform = Matrix::scaling(2.0, 2.0, 2.0);
        let _xs = s.intersect(r);

        assert_eq!(s.saved_ray.origin, RayTuple::point(0.0, 0.0, -2.5));
        assert_eq!(s.saved_ray.direction, RayTuple::vector(0.0, 0.0, 0.5));
    }

    #[test]
    fn intersecting_translated_shape_with_ray() {
        let r = Ray::new(
            RayTuple::point(0.0, 0.0, -5.0),
            RayTuple::vector(0.0, 0.0, 1.0),
        );
        let mut s = Shape::test_shape();
        s.transform = Matrix::translation(5.0, 0.0, 0.0);
        let _xs = s.intersect(r);

        assert_eq!(s.saved_ray.origin, RayTuple::point(-5.0, 0.0, -5.0));
        assert_eq!(s.saved_ray.direction, RayTuple::vector(0.0, 0.0, 1.0));
    }

    #[test]
    fn compute_normal_on_translated_shape() {
        let mut s = Shape::test_shape();
        s.transform = Matrix::translation(0.0, 1.0, 0.0);
        let n = s.normal_at(RayTuple::point(0.0, 1.70711, -0.70711));

        assert_eq!(n, RayTuple::vector(0.0, 0.70711, -0.70711));
    }

    #[test]
    fn compute_normal_on_transformed_shape() {
        let mut s = Shape::test_shape();
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
    fn normal_of_plane_is_constant_everywhere() {
        let p = Shape::plane();
        let n1 = p.normal_at(RayTuple::point(0.0, 0.0, 0.0));
        let n2 = p.normal_at(RayTuple::point(10.0, 0.0, -10.0));
        let n3 = p.normal_at(RayTuple::point(-5.0, 0.0, 150.0));

        let v = RayTuple::vector(0.0, 1.0, 0.0);
        assert_eq!(n1, v);
        assert_eq!(n2, v);
        assert_eq!(n3, v);
    }

    #[test]
    fn intersect_ray_parallel_to_plane() {
        let mut p = Shape::plane();
        let r = Ray::new(
            RayTuple::point(0.0, 10.0, 0.0),
            RayTuple::vector(0.0, 0.0, 1.0),
        );
        let xs = p.intersect(r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn intersect_coplanar_ray_plane() {
        let mut p = Shape::plane();
        let r = Ray::new(
            RayTuple::point(0.0, 0.0, 0.0),
            RayTuple::vector(0.0, 0.0, 1.0),
        );
        let xs = p.intersect(r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn ray_intersect_plane_from_above() {
        let mut p = Shape::plane();
        let r = Ray::new(
            RayTuple::point(0.0, 1.0, 0.0),
            RayTuple::vector(0.0, -1.0, 0.0),
        );
        let xs = p.intersect(r);

        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1.0);
        assert_eq!(xs[0].object, p);
    }

    #[test]
    fn ray_intersect_plane_from_below() {
        let mut p = Shape::plane();
        let r = Ray::new(
            RayTuple::point(0.0, -1.0, 0.0),
            RayTuple::vector(0.0, 1.0, 0.0),
        );
        let xs = p.intersect(r);

        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1.0);
        assert_eq!(xs[0].object, p);
    }
}
