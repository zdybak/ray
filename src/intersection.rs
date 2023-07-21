#![allow(dead_code)]
use crate::computations::Computations;
use crate::ray::Ray;
use crate::shape::Shape;

#[derive(Debug, Clone, Copy)]
pub struct Intersection {
    pub t: f64,
    pub object: Shape,
}

impl Intersection {
    pub fn new(t: f64, object: Shape) -> Intersection {
        Self { t, object }
    }

    pub fn hit(intersections: Vec<Intersection>) -> Option<Intersection> {
        let mut lowest_positive_i: Option<Intersection> = None;
        for i in intersections.into_iter() {
            if i.t >= 0.0 && lowest_positive_i == None {
                lowest_positive_i = Some(i);
            }
            match lowest_positive_i {
                Some(intersection) => {
                    if i.t >= 0.0 && intersection.t > i.t {
                        lowest_positive_i = Some(i);
                    }
                }
                None => continue,
            }
        }

        lowest_positive_i
    }

    pub fn prepare_computations(self, r: Ray, xs: Vec<Intersection>) -> Computations {
        let p = r.position(self.t);
        let eyev = -r.direction;
        let mut normalv = self.object.normal_at(p);
        let mut inside = false;
        if normalv.dot(eyev) < 0.0 {
            inside = true;
            normalv = -normalv;
        }
        let over_point = p + normalv * 0.00001;
        let reflectv = r.direction.reflect(normalv);

        let mut containers: Vec<Shape> = Vec::new();

        let mut n1: f64 = 1.0;
        let mut n2: f64 = 1.0;

        for i in xs {
            if self == i {
                if containers.len() == 0 {
                    n1 = 1.0;
                } else {
                    n1 = containers.last().unwrap().material.refractive_index;
                }
            }

            if let Some(shape_index) = containers.iter().position(|&s| s == i.object) {
                containers.remove(shape_index);
            } else {
                containers.push(i.object);
            }

            if self == i {
                if containers.len() == 0 {
                    n2 = 1.0;
                } else {
                    n2 = containers.last().unwrap().material.refractive_index;
                }
                break;
            }
        }

        Computations::new(
            self.t,
            self.object,
            p,
            over_point,
            eyev,
            normalv,
            inside,
            reflectv,
            n1,
            n2,
        )
    }
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t && self.object == other.object
    }
}

#[macro_export]
macro_rules! intersections {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec: Vec<Intersection> = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matrix::Matrix;
    use crate::raytuple::RayTuple;
    use crate::shape::{Shape, ShapeType};

    #[test]
    fn intersection_encapsulates_time_and_object() {
        let s = Shape::new(ShapeType::Sphere);
        let i = Intersection::new(3.5, s);

        assert_eq!(i.t, 3.5);
        assert_eq!(i.object, s);
    }

    #[test]
    fn aggregate_intersections() {
        let s = Shape::new(ShapeType::Sphere);
        let i1 = Intersection::new(1.0, s);
        let i2 = Intersection::new(2.0, s);
        let xs = intersections!(i1, i2);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 1.0);
        assert_eq!(xs[1].t, 2.0);
    }

    #[test]
    fn hit_with_all_positives() {
        let s = Shape::new(ShapeType::Sphere);
        let i1 = Intersection::new(1.0, s);
        let i2 = Intersection::new(2.0, s);
        let xs = intersections!(i2, i1);

        let i = Intersection::hit(xs).unwrap();
        assert_eq!(i, i1);
    }

    #[test]
    fn hit_with_some_negatives() {
        let s = Shape::new(ShapeType::Sphere);
        let i1 = Intersection::new(-1.0, s);
        let i2 = Intersection::new(1.0, s);
        let xs = intersections!(i2, i1);

        let i = Intersection::hit(xs).unwrap();
        assert_eq!(i, i2);
    }

    #[test]
    fn hit_with_all_negatives() {
        let s = Shape::new(ShapeType::Sphere);
        let i1 = Intersection::new(-2.0, s);
        let i2 = Intersection::new(-1.0, s);
        let xs = intersections!(i2, i1);

        let i = Intersection::hit(xs);
        match i {
            Some(_) => panic!("Test Failed to return none"),
            None => assert!(true),
        }
    }

    #[test]
    fn hit_is_always_lowest_positive() {
        let s = Shape::new(ShapeType::Sphere);
        let i1 = Intersection::new(5.0, s);
        let i2 = Intersection::new(7.0, s);
        let i3 = Intersection::new(-3.0, s);
        let i4 = Intersection::new(2.0, s);
        let xs = intersections!(i1, i2, i3, i4);

        let i = Intersection::hit(xs).unwrap();
        assert_eq!(i, i4);
    }

    #[test]
    fn precomputing_intersection_state() {
        let r = Ray::new(
            RayTuple::point(0.0, 0.0, -5.0),
            RayTuple::vector(0.0, 0.0, 1.0),
        );
        let shape = Shape::new(ShapeType::Sphere);
        let i = Intersection::new(4.0, shape);
        let xs: Vec<Intersection> = Vec::new();
        let comps = i.prepare_computations(r, xs);

        assert_eq!(comps.t, i.t);
        assert_eq!(comps.object, i.object);
        assert_eq!(comps.point, RayTuple::point(0.0, 0.0, -1.0));
        assert_eq!(comps.eyev, RayTuple::vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, RayTuple::vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn precomput_hit_on_outside() {
        let r = Ray::new(
            RayTuple::point(0.0, 0.0, -5.0),
            RayTuple::vector(0.0, 0.0, 1.0),
        );
        let shape = Shape::new(ShapeType::Sphere);
        let i = Intersection::new(4.0, shape);
        let xs: Vec<Intersection> = Vec::new();
        let comps = i.prepare_computations(r, xs);

        assert_eq!(comps.inside, false);
    }

    #[test]
    fn precomput_hit_on_inside() {
        let r = Ray::new(
            RayTuple::point(0.0, 0.0, 0.0),
            RayTuple::vector(0.0, 0.0, 1.0),
        );
        let shape = Shape::new(ShapeType::Sphere);
        let i = Intersection::new(1.0, shape);
        let xs: Vec<Intersection> = Vec::new();
        let comps = i.prepare_computations(r, xs);

        assert_eq!(comps.point, RayTuple::point(0.0, 0.0, 1.0));
        assert_eq!(comps.eyev, RayTuple::vector(0.0, 0.0, -1.0));
        assert!(comps.inside);
        assert_eq!(comps.normalv, RayTuple::vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn the_hit_should_offset_the_point() {
        let r = Ray::new(
            RayTuple::point(0.0, 0.0, -5.0),
            RayTuple::vector(0.0, 0.0, 1.0),
        );
        let mut shape = Shape::new(ShapeType::Sphere);
        shape.transform = Matrix::translation(0.0, 0.0, 1.0);
        let i = Intersection::new(5.0, shape);
        let xs: Vec<Intersection> = Vec::new();
        let comps = i.prepare_computations(r, xs);

        assert!(comps.over_point.z < (-f64::EPSILON / 2.0));
        assert!(comps.point.z > comps.over_point.z);
    }

    #[test]
    fn precompute_reflective_vector() {
        let shape = Shape::plane();
        let r = Ray::new(
            RayTuple::point(0.0, 1.0, -1.0),
            RayTuple::vector(0.0, -2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0),
        );
        let i = Intersection::new(2.0_f64.sqrt(), shape);
        let xs: Vec<Intersection> = Vec::new();
        let comps = i.prepare_computations(r, xs);

        assert_eq!(
            comps.reflectv,
            RayTuple::vector(0.0, 2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0)
        );
    }

    #[test]
    fn finding_n_at_various_intersections() {
        let mut a = Shape::glass_sphere();
        a.transform = Matrix::scaling(2.0, 2.0, 2.0);
        a.material.refractive_index = 1.5;

        let mut b = Shape::glass_sphere();
        b.transform = Matrix::translation(0.0, 0.0, -0.25);
        b.material.refractive_index = 2.0;

        let mut c = Shape::glass_sphere();
        c.transform = Matrix::translation(0.0, 0.0, 0.25);
        c.material.refractive_index = 2.5;

        let r = Ray::new(
            RayTuple::point(0.0, 0.0, -4.0),
            RayTuple::vector(0.0, 0.0, 1.0),
        );
        let xs1 = intersections!(
            Intersection::new(2.0, a),
            Intersection::new(2.75, b),
            Intersection::new(3.25, c),
            Intersection::new(4.75, b),
            Intersection::new(5.25, c),
            Intersection::new(6.0, a)
        );

        let xs2 = xs1.clone();
        let xs3 = xs1.clone();
        let xs4 = xs1.clone();
        let xs5 = xs1.clone();
        let xs6 = xs1.clone();

        let comps1 = xs1[0].prepare_computations(r, xs1);
        assert_eq!(comps1.n1, 1.0);
        assert_eq!(comps1.n2, 1.5);

        let comps2 = xs2[1].prepare_computations(r, xs2);
        assert_eq!(comps2.n1, 1.5);
        assert_eq!(comps2.n2, 2.0);

        let comps3 = xs3[2].prepare_computations(r, xs3);
        assert_eq!(comps3.n1, 2.0);
        assert_eq!(comps3.n2, 2.5);

        let comps4 = xs4[3].prepare_computations(r, xs4);
        assert_eq!(comps4.n1, 2.5);
        assert_eq!(comps4.n2, 2.5);

        let comps5 = xs5[4].prepare_computations(r, xs5);
        assert_eq!(comps5.n1, 2.5);
        assert_eq!(comps5.n2, 1.5);

        let comps6 = xs6[5].prepare_computations(r, xs6);
        assert_eq!(comps6.n1, 1.5);
        assert_eq!(comps6.n2, 1.0);
    }
}
