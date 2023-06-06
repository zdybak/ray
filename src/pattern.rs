#![allow(dead_code)]
use crate::color::Color;
use crate::matrix::Matrix;
use crate::raytuple::RayTuple;
use crate::shape::Shape;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PatternType {
    Stripe,
    Gradient,
    Test,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Pattern {
    pattern_type: PatternType,
    pub a: Color,
    pub b: Color,
    pub transform: Matrix,
}

impl Pattern {
    pub fn stripe_pattern(a: Color, b: Color) -> Self {
        Self {
            pattern_type: PatternType::Stripe,
            a,
            b,
            transform: Matrix::identity(),
        }
    }

    pub fn test_pattern() -> Self {
        Self {
            pattern_type: PatternType::Test,
            a: Color::new(0.0, 0.0, 0.0),
            b: Color::new(1.0, 1.0, 1.0),
            transform: Matrix::identity(),
        }
    }

    pub fn pattern_at(&self, point: RayTuple) -> Color {
        match self.pattern_type {
            PatternType::Stripe => {
                if point.x.floor() % 2.0 == 0.0 {
                    self.a
                } else {
                    self.b
                }
            }
            PatternType::Gradient => Color::new(0.0, 0.0, 0.0),
            PatternType::Test => Color::new(point.x, point.y, point.z),
        }
    }

    pub fn pattern_at_shape(&self, object: Shape, world_point: RayTuple) -> Color {
        let object_point = object.transform.inverse().unwrap() * world_point;
        let pattern_point = self.transform.inverse().unwrap() * object_point;

        self.pattern_at(pattern_point)
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
        assert_eq!(p.pattern_at(RayTuple::point(0.0, 0.0, 0.0)), white);
        assert_eq!(p.pattern_at(RayTuple::point(0.0, 1.0, 0.0)), white);
        assert_eq!(p.pattern_at(RayTuple::point(0.0, 2.0, 0.0)), white);
    }

    #[test]
    fn stripe_pattern_constant_z() {
        let white = Color::new(1.0, 1.0, 1.0);
        let black = Color::new(0.0, 0.0, 0.0);

        let p = Pattern::stripe_pattern(white, black);
        assert_eq!(p.pattern_at(RayTuple::point(0.0, 0.0, 0.0)), white);
        assert_eq!(p.pattern_at(RayTuple::point(0.0, 0.0, 1.0)), white);
        assert_eq!(p.pattern_at(RayTuple::point(0.0, 0.0, 2.0)), white);
    }

    #[test]
    fn stripe_pattern_alternates_x() {
        let white = Color::new(1.0, 1.0, 1.0);
        let black = Color::new(0.0, 0.0, 0.0);

        let p = Pattern::stripe_pattern(white, black);
        assert_eq!(p.pattern_at(RayTuple::point(0.0, 0.0, 0.0)), white);
        assert_eq!(p.pattern_at(RayTuple::point(0.9, 0.0, 0.0)), white);
        assert_eq!(p.pattern_at(RayTuple::point(1.0, 0.0, 0.0)), black);
        assert_eq!(p.pattern_at(RayTuple::point(-0.1, 0.0, 0.0)), black);
        assert_eq!(p.pattern_at(RayTuple::point(-1.0, 0.0, 0.0)), black);
        assert_eq!(p.pattern_at(RayTuple::point(-1.1, 0.0, 0.0)), white);
    }

    #[test]
    fn stripes_with_object_transform() {
        let mut object = Shape::sphere();
        object.transform = Matrix::scaling(2.0, 2.0, 2.0);

        let white = Color::new(1.0, 1.0, 1.0);
        let black = Color::new(0.0, 0.0, 0.0);

        let p = Pattern::stripe_pattern(white, black);
        let c = p.pattern_at_shape(object, RayTuple::point(1.5, 0.0, 0.0));
        assert_eq!(c, white);
    }

    #[test]
    fn stripes_with_pattern_transform() {
        let object = Shape::sphere();

        let white = Color::new(1.0, 1.0, 1.0);
        let black = Color::new(0.0, 0.0, 0.0);

        let mut p = Pattern::stripe_pattern(white, black);
        p.transform = Matrix::scaling(2.0, 2.0, 2.0);
        let c = p.pattern_at_shape(object, RayTuple::point(1.5, 0.0, 0.0));
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
        let c = p.pattern_at_shape(object, RayTuple::point(2.5, 0.0, 0.0));
        assert_eq!(c, white);
    }

    #[test]
    fn default_pattern_transform() {
        let pattern = Pattern::test_pattern();
        assert_eq!(pattern.transform, Matrix::identity());
    }

    #[test]
    fn pattern_transform_assign() {
        let mut pattern = Pattern::test_pattern();
        pattern.transform = Matrix::translation(1.0, 2.0, 3.0);
        assert_eq!(pattern.transform, Matrix::translation(1.0, 2.0, 3.0));
    }

    #[test]
    fn pattern_with_object_transform() {
        let mut shape = Shape::sphere();
        shape.transform = Matrix::scaling(2.0, 2.0, 2.0);
        let pattern = Pattern::test_pattern();
        let c = pattern.pattern_at_shape(shape, RayTuple::point(2.0, 3.0, 4.0));
        assert_eq!(c, Color::new(1.0, 1.5, 2.0));
    }

    #[test]
    fn pattern_with_pattern_transform() {
        let shape = Shape::sphere();
        let mut pattern = Pattern::test_pattern();
        pattern.transform = Matrix::scaling(2.0, 2.0, 2.0);
        let c = pattern.pattern_at_shape(shape, RayTuple::point(2.0, 3.0, 4.0));
        assert_eq!(c, Color::new(1.0, 1.5, 2.0));
    }

    #[test]
    fn pattern_with_both_transform() {
        let mut shape = Shape::sphere();
        shape.transform = Matrix::scaling(2.0, 2.0, 2.0);
        let mut pattern = Pattern::test_pattern();
        pattern.transform = Matrix::translation(0.5, 1.0, 1.5);
        let c = pattern.pattern_at_shape(shape, RayTuple::point(2.5, 3.0, 3.5));
        assert_eq!(c, Color::new(0.75, 0.5, 0.25));
    }
}
