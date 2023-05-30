#![allow(dead_code)]
use crate::color::Color;
use crate::matrix::Matrix;
use crate::raytuple::RayTuple;
use crate::shape::Shape;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Pattern {
    pub a: Color,
    pub b: Color,
    pub transform: Matrix,
}

impl Pattern {
    pub fn stripe_pattern(a: Color, b: Color) -> Self {
        Self {
            a,
            b,
            transform: Matrix::identity(),
        }
    }

    pub fn stripe_at(&self, point: RayTuple) -> Color {
        if point.x.floor() % 2.0 == 0.0 {
            self.a
        } else {
            self.b
        }
    }

    pub fn stripe_at_object(&self, object: Shape, world_point: RayTuple) -> Color {
        let object_point = object.transform.inverse().unwrap() * world_point;
        let pattern_point = self.transform.inverse().unwrap() * object_point;

        self.stripe_at(pattern_point)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;
    use crate::matrix::Matrix;
    use crate::shape::Shape;

    #[test]
    fn create_striped_pattern() {
        let white = Color::new(1.0, 1.0, 1.0);
        let black = Color::new(0.0, 0.0, 0.0);

        let p = Pattern::stripe_pattern(white, black);
        assert_eq!(p.a, white);
        assert_eq!(p.b, black);
    }

    #[test]
    fn stripe_pattern_constant_y() {
        let white = Color::new(1.0, 1.0, 1.0);
        let black = Color::new(0.0, 0.0, 0.0);

        let p = Pattern::stripe_pattern(white, black);
        assert_eq!(p.stripe_at(RayTuple::point(0.0, 0.0, 0.0)), white);
        assert_eq!(p.stripe_at(RayTuple::point(0.0, 1.0, 0.0)), white);
        assert_eq!(p.stripe_at(RayTuple::point(0.0, 2.0, 0.0)), white);
    }

    #[test]
    fn stripe_pattern_constant_z() {
        let white = Color::new(1.0, 1.0, 1.0);
        let black = Color::new(0.0, 0.0, 0.0);

        let p = Pattern::stripe_pattern(white, black);
        assert_eq!(p.stripe_at(RayTuple::point(0.0, 0.0, 0.0)), white);
        assert_eq!(p.stripe_at(RayTuple::point(0.0, 0.0, 1.0)), white);
        assert_eq!(p.stripe_at(RayTuple::point(0.0, 0.0, 2.0)), white);
    }

    #[test]
    fn stripe_pattern_alternates_x() {
        let white = Color::new(1.0, 1.0, 1.0);
        let black = Color::new(0.0, 0.0, 0.0);

        let p = Pattern::stripe_pattern(white, black);
        assert_eq!(p.stripe_at(RayTuple::point(0.0, 0.0, 0.0)), white);
        assert_eq!(p.stripe_at(RayTuple::point(0.9, 0.0, 0.0)), white);
        assert_eq!(p.stripe_at(RayTuple::point(1.0, 0.0, 0.0)), black);
        assert_eq!(p.stripe_at(RayTuple::point(-0.1, 0.0, 0.0)), black);
        assert_eq!(p.stripe_at(RayTuple::point(-1.0, 0.0, 0.0)), black);
        assert_eq!(p.stripe_at(RayTuple::point(-1.1, 0.0, 0.0)), white);
    }

    #[test]
    fn stripes_with_object_transform() {
        let mut object = Shape::sphere();
        object.transform = Matrix::scaling(2.0, 2.0, 2.0);

        let white = Color::new(1.0, 1.0, 1.0);
        let black = Color::new(0.0, 0.0, 0.0);

        let p = Pattern::stripe_pattern(white, black);
        let c = p.stripe_at_object(object, RayTuple::point(1.5, 0.0, 0.0));
        assert_eq!(c, white);
    }

    #[test]
    fn stripes_with_pattern_transform() {
        let object = Shape::sphere();

        let white = Color::new(1.0, 1.0, 1.0);
        let black = Color::new(0.0, 0.0, 0.0);

        let mut p = Pattern::stripe_pattern(white, black);
        p.transform = Matrix::scaling(2.0, 2.0, 2.0);
        let c = p.stripe_at_object(object, RayTuple::point(1.5, 0.0, 0.0));
        assert_eq!(c, white);
    }

    #[test]
    fn stripes_with_both_transform() {
        let mut object = Shape::sphere();
        object.transform = Matrix::scaling(2.0, 2.0, 2.0);

        let white = Color::new(1.0, 1.0, 1.0);
        let black = Color::new(0.0, 0.0, 0.0);

        let mut p = Pattern::stripe_pattern(white, black);
        p.transform = Matrix::translation(0.5, 0.0, 0.0);
        let c = p.stripe_at_object(object, RayTuple::point(2.5, 0.0, 0.0));
        assert_eq!(c, white);
    }
}
