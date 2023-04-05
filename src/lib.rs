use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug)]
pub struct RayTuple {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl RayTuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }

    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }

    pub fn point(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z, w: 1.0 }
    }

    pub fn vector(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z, w: 0.0 }
    }

    pub fn is_a_point(&self) -> bool {
        self.w == 1.0
    }

    pub fn is_a_vector(&self) -> bool {
        self.w == 0.0
    }
}

impl PartialEq for RayTuple {
    fn eq(&self, other: &Self) -> bool {
        let epsilon: f64 = 0.00001;

        f64::abs(self.x - other.x) < epsilon
            && f64::abs(self.y - other.y) < epsilon
            && f64::abs(self.z - other.z) < epsilon
    }
}

impl Add for RayTuple {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Sub for RayTuple {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl Neg for RayTuple {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Mul<f64> for RayTuple {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl Div<f64> for RayTuple {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //Pg.4 Scenario: A tuple with w=1.0 is a point
    #[test]
    fn point_creates_tuple_with_w1() {
        let a = RayTuple::point(4.3, -4.2, 3.1);
        assert_eq!(4.3, a.x);
        assert_eq!(-4.2, a.y);
        assert_eq!(3.1, a.z);
        assert_eq!(1.0, a.w);
        assert!(a.is_a_point());
        assert!(!a.is_a_vector());
    }

    //Pg.4 Scenario: A tuple with w=0.0 is a vector
    #[test]
    fn vector_creates_tuple_withw0() {
        let b = RayTuple::vector(4.3, -4.2, 3.1);
        assert_eq!(4.3, b.x);
        assert_eq!(-4.2, b.y);
        assert_eq!(3.1, b.z);
        assert_eq!(0.0, b.w);
        assert!(b.is_a_vector());
        assert!(!b.is_a_point());
    }

    //Test equality of points within 0.00001 epsilon
    #[test]
    fn points_are_equal_within_epsilon() {
        let a = RayTuple::point(1.0, 1.0, 1.0);
        let b = RayTuple::point(1.000001, 1.000001, 0.999999);

        assert_eq!(a, b);
    }

    //Test equality of vectors within 0.00001 epsilon
    #[test]
    fn vectors_are_equal_within_epsilon() {
        let a = RayTuple::vector(1.0, 1.0, 1.0);
        let b = RayTuple::vector(1.000001, 1.000001, 0.999999);

        assert_eq!(a, b);
    }

    //Pg.6 Scenario: Adding two tuples
    #[test]
    fn adding_two_tuples() {
        let a1 = RayTuple::point(3.0, -2.0, 5.0);
        let a2 = RayTuple::vector(-2.0, 3.0, 1.0);
        assert_eq!(a1 + a2, RayTuple::point(1.0, 1.0, 6.0));
    }

    //Pg.7 Scenario: Subtract two points
    #[test]
    fn subtract_two_points() {
        let p1 = RayTuple::point(3.0, 2.0, 1.0);
        let p2 = RayTuple::point(5.0, 6.0, 7.0);
        let ps = p1 - p2;
        assert_eq!(ps, RayTuple::vector(-2.0, -4.0, -6.0));
        assert!(ps.is_a_vector());
    }

    #[test]
    fn subtract_vector_from_point() {
        let p = RayTuple::point(3.0, 2.0, 1.0);
        let v = RayTuple::vector(5.0, 6.0, 7.0);
        let ps = p - v;
        assert_eq!(ps, RayTuple::point(-2.0, -4.0, -6.0));
        assert!(ps.is_a_point());
    }

    #[test]
    fn subtract_two_vectors() {
        let v1 = RayTuple::vector(3.0, 2.0, 1.0);
        let v2 = RayTuple::vector(5.0, 6.0, 7.0);
        let vs = v1 - v2;
        assert_eq!(vs, RayTuple::vector(-2.0, -4.0, -6.0));
        assert!(vs.is_a_vector());
    }

    #[test]
    fn subtract_vector_from_zero_vector() {
        let zero = RayTuple::vector(0.0, 0.0, 0.0);
        let v = RayTuple::vector(1.0, -2.0, 3.0);
        let vz = zero - v;
        assert_eq!(vz, RayTuple::vector(-1.0, 2.0, -3.0));
        assert!(vz.is_a_vector());
    }

    //Pg. 7 Negating a tuple
    #[test]
    fn negating_a_tuple() {
        let a = RayTuple {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };
        assert_eq!(
            -a,
            RayTuple {
                x: -1.0,
                y: 2.0,
                z: -3.0,
                w: 4.0
            }
        );
    }

    //Pg.8 Multiply a tuple by a scaler
    #[test]
    fn multiply_tuple_by_scalar() {
        let a = RayTuple::new(1.0, -2.0, 3.0, -4.0);
        let m = a * 3.5;
        assert_eq!(m, RayTuple::new(3.5, -7.0, 10.5, -14.0));

        let a = RayTuple::new(1.0, -2.0, 3.0, -4.0);
        let m = a * 0.5;
        assert_eq!(m, RayTuple::new(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn divide_tuple_by_scaler() {
        let a = RayTuple::new(1.0, -2.0, 3.0, -4.0);
        let d = a / 2.0;
        assert_eq!(d, RayTuple::new(0.5, -1.0, 1.5, -2.0));
    }
}
