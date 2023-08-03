#![allow(dead_code)]
use crate::camera::Camera;
use crate::color::Color;
use crate::intersection::Intersection;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::pattern::Pattern;
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
    Cube,
    Cylinder,
}

#[derive(Debug, Clone, Copy)]
pub struct Shape {
    id: Uuid,
    shape_type: ShapeType,
    pub transform: Matrix,
    pub material: Material,
    pub saved_ray: Ray,
    pub minimum: f64,
    pub maximum: f64,
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
            minimum: f64::NEG_INFINITY,
            maximum: f64::INFINITY,
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
            minimum: f64::NEG_INFINITY,
            maximum: f64::INFINITY,
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
            minimum: f64::NEG_INFINITY,
            maximum: f64::INFINITY,
        }
    }

    pub fn glass_sphere() -> Self {
        let mut s = Self::sphere();
        s.material.transparency = 1.0;
        s.material.refractive_index = 1.5;

        s
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
            minimum: f64::NEG_INFINITY,
            maximum: f64::INFINITY,
        }
    }

    pub fn cube() -> Self {
        Self {
            id: Uuid::new_v4(),
            shape_type: ShapeType::Cube,
            transform: Matrix::identity(),
            material: Material::new(),
            saved_ray: Ray::new(
                RayTuple::point(0.0, 0.0, 0.0),
                RayTuple::vector(0.0, 0.0, 0.0),
            ),
            minimum: f64::NEG_INFINITY,
            maximum: f64::INFINITY,
        }
    }

    pub fn cylinder() -> Self {
        Self {
            id: Uuid::new_v4(),
            shape_type: ShapeType::Cylinder,
            transform: Matrix::identity(),
            material: Material::new(),
            saved_ray: Ray::new(
                RayTuple::point(0.0, 0.0, 0.0),
                RayTuple::vector(0.0, 0.0, 0.0),
            ),
            minimum: f64::NEG_INFINITY,
            maximum: f64::INFINITY,
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
                if self.saved_ray.direction.y.abs() < epsilon {
                    return intersections;
                }

                let t = -self.saved_ray.origin.y / self.saved_ray.direction.y;

                intersections.push(Intersection::new(t, *self));
                intersections
            }
            ShapeType::Test => intersections,
            ShapeType::Cube => {
                //because cube is using the transformed ray, I think we might run into issues because we aren't doing local intersect technically?
                let xaxis: (f64, f64) =
                    Self::check_axis(self.saved_ray.origin.x, self.saved_ray.direction.x);
                let yaxis: (f64, f64) =
                    Self::check_axis(self.saved_ray.origin.y, self.saved_ray.direction.y);
                let zaxis: (f64, f64) =
                    Self::check_axis(self.saved_ray.origin.z, self.saved_ray.direction.z);

                let tmin = if xaxis.0 > yaxis.0 {
                    if xaxis.0 > zaxis.0 {
                        xaxis.0
                    } else {
                        zaxis.0
                    }
                } else {
                    if yaxis.0 > zaxis.0 {
                        yaxis.0
                    } else {
                        zaxis.0
                    }
                };
                let tmax = if xaxis.1 < yaxis.1 {
                    if xaxis.1 < zaxis.1 {
                        xaxis.1
                    } else {
                        zaxis.1
                    }
                } else {
                    if yaxis.1 < zaxis.1 {
                        yaxis.1
                    } else {
                        zaxis.1
                    }
                };

                if tmin > tmax {
                    return intersections;
                }

                intersections.push(Intersection::new(tmin, *self));
                intersections.push(Intersection::new(tmax, *self));
                intersections
            }
            ShapeType::Cylinder => {
                let epsilon: f64 = 0.00001;
                let a = self.saved_ray.direction.x.powf(2.0) + self.saved_ray.direction.z.powf(2.0);

                if a <= epsilon {
                    return intersections;
                }

                let b = 2.0 * self.saved_ray.origin.x * self.saved_ray.direction.x
                    + 2.0 * self.saved_ray.origin.z * self.saved_ray.direction.z;
                let c = self.saved_ray.origin.x.powf(2.0) + self.saved_ray.origin.z.powf(2.0) - 1.0;

                let disc = b.powf(2.0) - 4.0 * a * c;

                if disc < 0.0 {
                    return intersections;
                } else {
                    let mut t0 = (-b - disc.sqrt()) / (2.0 * a);
                    let mut t1 = (-b + disc.sqrt()) / (2.0 * a);

                    if t0 > t1 {
                        (t0, t1) = (t1, t0);
                    }

                    let y0 = self.saved_ray.origin.y + t0 * self.saved_ray.direction.y;
                    if self.minimum < y0 && y0 < self.maximum {
                        intersections.push(Intersection::new(t0, *self));
                    }

                    let y1 = self.saved_ray.origin.y + t1 * self.saved_ray.direction.y;
                    if self.minimum < y1 && y1 < self.maximum {
                        intersections.push(Intersection::new(t1, *self));
                    }

                    intersections
                }
            }
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
            ShapeType::Cube => {
                let x_abs = object_point.x.abs();
                let y_abs = object_point.y.abs();
                let z_abs = object_point.z.abs();

                if x_abs >= y_abs {
                    if x_abs >= z_abs {
                        RayTuple::vector(object_point.x, 0.0, 0.0)
                    } else {
                        RayTuple::vector(0.0, 0.0, object_point.z)
                    }
                } else {
                    if y_abs >= z_abs {
                        RayTuple::vector(0.0, object_point.y, 0.0)
                    } else {
                        RayTuple::vector(0.0, 0.0, object_point.z)
                    }
                }
            }
            ShapeType::Cylinder => RayTuple::vector(object_point.x, 0.0, object_point.z),
        }
    }

    fn check_axis(origin: f64, direction: f64) -> (f64, f64) {
        let epsilon: f64 = 0.00001;
        let tmin_numerator = -1.0 - origin;
        let tmax_numerator = 1.0 - origin;
        let tmin: f64;
        let tmax: f64;

        if direction.abs() >= epsilon {
            tmin = tmin_numerator / direction;
            tmax = tmax_numerator / direction;
        } else {
            tmin = tmin_numerator * f64::INFINITY;
            tmax = tmax_numerator * f64::INFINITY;
        }

        if tmin > tmax {
            (tmax, tmin)
        } else {
            (tmin, tmax)
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
    middle.material.pattern = Some(Pattern::stripe_pattern(
        Color::new(0.1, 1.0, 0.5),
        Color::new(0.9, 0.0, 0.5),
    ));
    middle.material.color = Color::new(0.1, 1.0, 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;

    let mut right = Shape::sphere();
    right.transform = Matrix::translation(1.5, 0.5, -0.5) * Matrix::scaling(0.5, 0.5, 0.5);
    right.material.pattern = Some(Pattern::stripe_pattern(
        Color::new(0.5, 1.0, 0.1),
        Color::new(0.5, 0.0, 0.9),
    ));
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
    let mut c = Camera::new(1920, 1080, FRAC_PI_3);
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

    #[test]
    fn glass_sphere_test() {
        let s = Shape::glass_sphere();

        assert_eq!(s.transform, Matrix::identity());
        assert_eq!(s.material.transparency, 1.0);
        assert_eq!(s.material.refractive_index, 1.5);
    }

    #[test]
    fn ray_intersects_cube() {
        let mut c = Shape::cube();

        let test_tuples: Vec<(RayTuple, RayTuple, f64, f64)> = vec![
            (
                RayTuple::point(5.0, 0.5, 0.0),
                RayTuple::vector(-1.0, 0.0, 0.0),
                4.0,
                6.0,
            ),
            (
                RayTuple::point(-5.0, 0.5, 0.0),
                RayTuple::vector(1.0, 0.0, 0.0),
                4.0,
                6.0,
            ),
            (
                RayTuple::point(0.5, 5.0, 0.0),
                RayTuple::vector(0.0, -1.0, 0.0),
                4.0,
                6.0,
            ),
            (
                RayTuple::point(0.5, -5.0, 0.0),
                RayTuple::vector(0.0, 1.0, 0.0),
                4.0,
                6.0,
            ),
            (
                RayTuple::point(0.5, 0.0, 5.0),
                RayTuple::vector(0.0, 0.0, -1.0),
                4.0,
                6.0,
            ),
            (
                RayTuple::point(0.5, 0.0, -5.0),
                RayTuple::vector(0.0, 0.0, 1.0),
                4.0,
                6.0,
            ),
            (
                RayTuple::point(0.0, 0.5, 0.0),
                RayTuple::vector(0.0, 0.0, 1.0),
                -1.0,
                1.0,
            ),
        ];

        for test in test_tuples {
            let r = Ray::new(test.0, test.1);
            let xs: Vec<Intersection> = c.intersect(r);

            assert_eq!(xs.len(), 2);
            assert_eq!(xs[0].t, test.2);
            assert_eq!(xs[1].t, test.3);
        }
    }

    #[test]
    fn ray_misses_cube() {
        let mut c = Shape::cube();

        let test_tuples: Vec<(RayTuple, RayTuple)> = vec![
            (
                RayTuple::point(-2.0, 0.0, 0.0),
                RayTuple::vector(0.2673, 0.5345, 0.8018),
            ),
            (
                RayTuple::point(0.0, -2.0, 0.0),
                RayTuple::vector(0.8018, 0.2673, 0.5345),
            ),
            (
                RayTuple::point(0.0, 0.0, -2.0),
                RayTuple::vector(0.5345, 0.8018, 0.2673),
            ),
            (
                RayTuple::point(2.0, 0.0, 2.0),
                RayTuple::vector(0.0, 0.0, -1.0),
            ),
            (
                RayTuple::point(0.0, 2.0, 2.0),
                RayTuple::vector(0.0, -1.0, 0.0),
            ),
            (
                RayTuple::point(2.0, 2.0, 0.0),
                RayTuple::vector(-1.0, 0.0, 0.0),
            ),
        ];

        for test in test_tuples {
            let r = Ray::new(test.0, test.1);
            let xs: Vec<Intersection> = c.intersect(r);

            assert_eq!(xs.len(), 0);
        }
    }

    #[test]
    fn normal_on_surface_of_cube() {
        let c = Shape::cube();

        let test_tuples: Vec<(RayTuple, RayTuple)> = vec![
            (
                RayTuple::point(1.0, 0.5, -0.8),
                RayTuple::vector(1.0, 0.0, 0.0),
            ),
            (
                RayTuple::point(-1.0, -0.2, 0.9),
                RayTuple::vector(-1.0, 0.0, 0.0),
            ),
            (
                RayTuple::point(-0.4, 1.0, -0.1),
                RayTuple::vector(0.0, 1.0, 0.0),
            ),
            (
                RayTuple::point(0.3, -1.0, -0.7),
                RayTuple::vector(0.0, -1.0, 0.0),
            ),
            (
                RayTuple::point(-0.6, 0.3, 1.0),
                RayTuple::vector(0.0, 0.0, 1.0),
            ),
            (
                RayTuple::point(0.4, 0.4, -1.0),
                RayTuple::vector(0.0, 0.0, -1.0),
            ),
            (
                RayTuple::point(1.0, 1.0, 1.0),
                RayTuple::vector(1.0, 0.0, 0.0),
            ),
            (
                RayTuple::point(-1.0, -1.0, -1.0),
                RayTuple::vector(-1.0, 0.0, 0.0),
            ),
        ];

        for test in test_tuples {
            let p = test.0;
            let normal = c.normal_at(p);

            assert_eq!(normal, test.1);
        }
    }

    #[test]
    fn ray_misses_cylinder() {
        let mut cyl = Shape::cylinder();

        let test_tuples: Vec<(RayTuple, RayTuple)> = vec![
            (
                RayTuple::point(1.0, 0.0, 0.0),
                RayTuple::vector(0.0, 1.0, 0.0),
            ),
            (
                RayTuple::point(0.0, 0.0, 0.0),
                RayTuple::vector(0.0, 1.0, 0.0),
            ),
            (
                RayTuple::point(0.0, 0.0, -5.0),
                RayTuple::vector(1.0, 1.0, 1.0),
            ),
        ];

        for test in test_tuples {
            let direction = test.1.normalize();
            let r = Ray::new(test.0, direction);
            let xs = cyl.intersect(r);

            assert_eq!(xs.len(), 0);
        }
    }

    #[test]
    fn ray_hits_cylinder() {
        let mut cyl = Shape::cylinder();

        let test_tuples: Vec<(RayTuple, RayTuple, f64, f64)> = vec![
            (
                RayTuple::point(1.0, 0.0, -5.0),
                RayTuple::vector(0.0, 0.0, 1.0),
                5.0,
                5.0,
            ),
            (
                RayTuple::point(0.0, 0.0, -5.0),
                RayTuple::vector(0.0, 0.0, 1.0),
                4.0,
                6.0,
            ),
            (
                RayTuple::point(0.5, 0.0, -5.0),
                RayTuple::vector(0.1, 1.0, 1.0),
                6.80798191702732,
                7.088723439378861,
            ),
        ];

        for test in test_tuples {
            let direction = test.1.normalize();
            let r = Ray::new(test.0, direction);
            let xs = cyl.intersect(r);

            assert_eq!(xs[0].t, test.2);
            assert_eq!(xs[1].t, test.3);
        }
    }

    #[test]
    fn normal_of_cylinder() {
        let cyl = Shape::cylinder();

        let test_tuples: Vec<(RayTuple, RayTuple)> = vec![
            (
                RayTuple::point(1.0, 0.0, 0.0),
                RayTuple::vector(1.0, 0.0, 0.0),
            ),
            (
                RayTuple::point(0.0, 5.0, -1.0),
                RayTuple::vector(0.0, 0.0, -1.0),
            ),
            (
                RayTuple::point(0.0, -2.0, 1.0),
                RayTuple::vector(0.0, 0.0, 1.0),
            ),
            (
                RayTuple::point(-1.0, 1.0, 0.0),
                RayTuple::vector(-1.0, 0.0, 0.0),
            ),
        ];

        for test in test_tuples {
            let n = cyl.normal_at(test.0);

            assert_eq!(n, test.1);
        }
    }

    #[test]
    fn default_min_and_max_of_cylinder() {
        let cyl = Shape::cylinder();

        assert_eq!(cyl.minimum, f64::NEG_INFINITY);
        assert_eq!(cyl.maximum, f64::INFINITY);
    }

    #[test]
    fn intersecting_constrained_cylinder() {
        let mut cyl = Shape::cylinder();
        cyl.minimum = 1.0;
        cyl.maximum = 2.0;

        let test_tuples: Vec<(RayTuple, RayTuple, usize)> = vec![
            (
                RayTuple::point(0.0, 1.5, 0.0),
                RayTuple::vector(0.1, 1.0, 0.0),
                0,
            ),
            (
                RayTuple::point(0.0, 3.0, -5.0),
                RayTuple::vector(0.0, 0.0, 1.0),
                0,
            ),
            (
                RayTuple::point(0.0, 0.0, -5.0),
                RayTuple::vector(0.0, 0.0, 1.0),
                0,
            ),
            (
                RayTuple::point(0.0, 2.0, -5.0),
                RayTuple::vector(0.0, 0.0, 1.0),
                0,
            ),
            (
                RayTuple::point(0.0, 1.0, -5.0),
                RayTuple::vector(0.0, 0.0, 1.0),
                0,
            ),
            (
                RayTuple::point(0.0, 1.5, -2.0),
                RayTuple::vector(0.0, 0.0, 1.0),
                2,
            ),
        ];

        for test in test_tuples {
            let direction = test.1.normalize();
            let r = Ray::new(test.0, direction);
            let xs = cyl.intersect(r);

            assert_eq!(xs.len(), test.2);
        }
    }
}
