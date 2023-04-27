#![allow(dead_code)]
use crate::matrix::Matrix;
use crate::raytuple::RayTuple;

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

    pub fn transform(self, m: Matrix) -> Self {
        Self {
            origin: m * self.origin,
            direction: m * self.direction,
        }
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
    fn translate_ray() {
        let r = Ray::new(
            RayTuple::point(1.0, 2.0, 3.0),
            RayTuple::vector(0.0, 1.0, 0.0),
        );
        let m = Matrix::translation(3.0, 4.0, 5.0);
        let r2 = r.transform(m);

        assert_eq!(r2.origin, RayTuple::point(4.0, 6.0, 8.0));
        assert_eq!(r2.direction, RayTuple::vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn scale_ray() {
        let r = Ray::new(
            RayTuple::point(1.0, 2.0, 3.0),
            RayTuple::vector(0.0, 1.0, 0.0),
        );
        let m = Matrix::scaling(2.0, 3.0, 4.0);
        let r2 = r.transform(m);

        assert_eq!(r2.origin, RayTuple::point(2.0, 6.0, 12.0));
        assert_eq!(r2.direction, RayTuple::vector(0.0, 3.0, 0.0));
    }
}
