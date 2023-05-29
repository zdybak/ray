#![allow(dead_code)]
use crate::color::Color;
use crate::raytuple::RayTuple;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Pattern {
    pub a: Color,
    pub b: Color,
}

impl Pattern {
    pub fn stripe_pattern(a: Color, b: Color) -> Self {
        Self { a, b }
    }

    pub fn stripe_at(&self, point: RayTuple) -> Color {
        if point.x.floor() % 2.0 == 0.0 {
            self.a
        } else {
            self.b
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;

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
}
