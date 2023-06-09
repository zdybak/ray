#![allow(dead_code)]
use crate::camera::Camera;
use crate::color::Color;
use crate::matrix::Matrix;
use crate::raytuple::RayTuple;
use crate::shape::Shape;
use crate::world::World;
use std::f64::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_8};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PatternType {
    Stripe,
    Gradient,
    Test,
    Ring,
    Checker,
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

    pub fn gradient_pattern(a: Color, b: Color) -> Self {
        Self {
            pattern_type: PatternType::Gradient,
            a,
            b,
            transform: Matrix::identity(),
        }
    }

    pub fn ring_pattern(a: Color, b: Color) -> Self {
        Self {
            pattern_type: PatternType::Ring,
            a,
            b,
            transform: Matrix::identity(),
        }
    }

    pub fn checkers_pattern(a: Color, b: Color) -> Self {
        Self {
            pattern_type: PatternType::Checker,
            a,
            b,
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
            PatternType::Gradient => {
                let distance = self.b - self.a;
                let fraction = point.x - point.x.floor();

                self.a + distance * fraction
            }
            PatternType::Test => Color::new(point.x, point.y, point.z),
            PatternType::Ring => {
                let distance = (point.x.powf(2.0) + point.z.powf(2.0)).sqrt().floor() % 2.0;
                if distance == 0.0 {
                    self.a
                } else {
                    self.b
                }
            }
            PatternType::Checker => {
                let distance = (point.x.floor() + point.y.floor() + point.z.floor()) % 2.0;
                if distance == 0.0 {
                    self.a
                } else {
                    self.b
                }
            }
        }
    }

    pub fn pattern_at_shape(&self, object: Shape, world_point: RayTuple) -> Color {
        let object_point = object.transform.inverse().unwrap() * world_point;
        let pattern_point = self.transform.inverse().unwrap() * object_point;

        self.pattern_at(pattern_point)
    }
}

pub fn chapter_ten_patterns() {
    let mut floor = Shape::plane();
    floor.material.color = Color::new(1.0, 0.9, 0.9);
    floor.material.specular = 0.0;

    floor.material.pattern = Some(Pattern::checkers_pattern(
        Color::new(1.0, 0.9, 0.9),
        Color::new(0.6, 0.5, 0.5),
    ));

    let mut middle = Shape::sphere();
    middle.transform = Matrix::translation(-0.5, 1.0, 0.5);
    middle.material.color = Color::new(0.1, 1.0, 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;

    let mut middle_ring_pattern =
        Pattern::ring_pattern(Color::new(0.9, 1.0, 1.0), Color::new(1.0, 0.0, 0.0));
    middle_ring_pattern.transform = Matrix::scaling(0.15, 0.15, 0.15)
        * Matrix::rotation_x(FRAC_PI_2)
        * Matrix::rotation_y(FRAC_PI_8);
    middle.material.pattern = Some(middle_ring_pattern);

    let mut right = Shape::sphere();
    right.transform = Matrix::translation(1.5, 0.5, -0.5) * Matrix::scaling(0.5, 0.5, 0.5);
    right.material.color = Color::new(0.5, 1.0, 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;

    let mut right_gradient_pattern =
        Pattern::gradient_pattern(Color::new(0.5, 1.0, 0.1), Color::new(0.65, 0.2, 0.2));
    right_gradient_pattern.transform = Matrix::scaling(1.2, 1.2, 1.2)
        * Matrix::rotation_x(0.14)
        * Matrix::rotation_y(FRAC_PI_2 + 0.14);
    right.material.pattern = Some(right_gradient_pattern);

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
    canvas.save_ppm("chapter10.ppm");
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

    #[test]
    fn gradient_linearly_interpolates_between_colors() {
        let white = Color::new(1.0, 1.0, 1.0);
        let black = Color::new(0.0, 0.0, 0.0);

        let pattern = Pattern::gradient_pattern(white, black);

        assert_eq!(pattern.pattern_at(RayTuple::point(0.0, 0.0, 0.0)), white);
        assert_eq!(
            pattern.pattern_at(RayTuple::point(0.25, 0.0, 0.0)),
            Color::new(0.75, 0.75, 0.75)
        );
        assert_eq!(
            pattern.pattern_at(RayTuple::point(0.5, 0.0, 0.0)),
            Color::new(0.5, 0.5, 0.5)
        );
        assert_eq!(
            pattern.pattern_at(RayTuple::point(0.75, 0.0, 0.0)),
            Color::new(0.25, 0.25, 0.25)
        );
    }

    #[test]
    fn ring_should_extend_in_x_and_z() {
        let white = Color::new(1.0, 1.0, 1.0);
        let black = Color::new(0.0, 0.0, 0.0);

        let pattern = Pattern::ring_pattern(white, black);

        assert_eq!(pattern.pattern_at(RayTuple::point(0.0, 0.0, 0.0)), white);
        assert_eq!(pattern.pattern_at(RayTuple::point(1.0, 0.0, 0.0)), black);
        assert_eq!(pattern.pattern_at(RayTuple::point(0.0, 0.0, 1.0)), black);
        assert_eq!(
            pattern.pattern_at(RayTuple::point(0.708, 0.0, 0.708)),
            black
        );
    }

    #[test]
    fn checkers_repeat_in_x() {
        let white = Color::new(1.0, 1.0, 1.0);
        let black = Color::new(0.0, 0.0, 0.0);

        let pattern = Pattern::checkers_pattern(white, black);

        assert_eq!(pattern.pattern_at(RayTuple::point(0.0, 0.0, 0.0)), white);
        assert_eq!(pattern.pattern_at(RayTuple::point(0.99, 0.0, 0.0)), white);
        assert_eq!(pattern.pattern_at(RayTuple::point(1.01, 0.0, 0.0)), black);
    }

    #[test]
    fn checkers_repeat_in_y() {
        let white = Color::new(1.0, 1.0, 1.0);
        let black = Color::new(0.0, 0.0, 0.0);

        let pattern = Pattern::checkers_pattern(white, black);

        assert_eq!(pattern.pattern_at(RayTuple::point(0.0, 0.0, 0.0)), white);
        assert_eq!(pattern.pattern_at(RayTuple::point(0.0, 0.99, 0.0)), white);
        assert_eq!(pattern.pattern_at(RayTuple::point(0.0, 1.01, 0.0)), black);
    }

    #[test]
    fn checkers_repeat_in_z() {
        let white = Color::new(1.0, 1.0, 1.0);
        let black = Color::new(0.0, 0.0, 0.0);

        let pattern = Pattern::checkers_pattern(white, black);

        assert_eq!(pattern.pattern_at(RayTuple::point(0.0, 0.0, 0.0)), white);
        assert_eq!(pattern.pattern_at(RayTuple::point(0.0, 0.0, 0.99)), white);
        assert_eq!(pattern.pattern_at(RayTuple::point(0.0, 0.0, 1.01)), black);
    }
}
