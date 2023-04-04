use std::ops::{Add,Sub};

#[derive(Debug)]
pub struct RayTuple {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl RayTuple {
    pub fn point(x: f64, y: f64, z: f64) -> Self {
        Self {
            x,
            y,
            z,
            w: 1.0,
        }
    }

    pub fn vector(x: f64, y: f64, z: f64) -> Self {
        Self {
            x,
            y,
            z,
            w: 0.0,
        }
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
        
        f64::abs( self.x - other.x ) < epsilon &&
        f64::abs( self.y - other.y ) < epsilon &&
        f64::abs( self.z - other.z ) < epsilon
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
        assert_eq!( a1 + a2, RayTuple::point(1.0, 1.0, 6.0));
    }

    //Pg.7 Scenario: Subtract two points
    #[test]
    fn subtract_two_points() {
        let p1 = RayTuple::point(3.0, 2.0, 1.0);
        let p2 = RayTuple::point(5.0, 6.0, 7.0);
        let ps = p1 - p2;
        assert_eq!( ps, RayTuple::vector(-2.0, -4.0, -6.0));
        assert!(ps.is_a_vector());

    }
}