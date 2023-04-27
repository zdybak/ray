#![allow(dead_code)]
use crate::sphere::Sphere;

#[derive(Debug, Clone, Copy)]
pub struct Intersection {
    pub t: f64,
    pub object: Sphere,
}

impl Intersection {
    pub fn new(t: f64, object: Sphere) -> Intersection {
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

    #[test]
    fn intersection_encapsulates_time_and_object() {
        let s = Sphere::new();
        let i = Intersection::new(3.5, s);

        assert_eq!(i.t, 3.5);
        assert_eq!(i.object, s);
    }

    #[test]
    fn aggregate_intersections() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, s);
        let i2 = Intersection::new(2.0, s);
        let xs = intersections!(i1, i2);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 1.0);
        assert_eq!(xs[1].t, 2.0);
    }

    #[test]
    fn hit_with_all_positives() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, s);
        let i2 = Intersection::new(2.0, s);
        let xs = intersections!(i2, i1);

        let i = Intersection::hit(xs).unwrap();
        assert_eq!(i, i1);
    }

    #[test]
    fn hit_with_some_negatives() {
        let s = Sphere::new();
        let i1 = Intersection::new(-1.0, s);
        let i2 = Intersection::new(1.0, s);
        let xs = intersections!(i2, i1);

        let i = Intersection::hit(xs).unwrap();
        assert_eq!(i, i2);
    }

    #[test]
    fn hit_with_all_negatives() {
        let s = Sphere::new();
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
        let s = Sphere::new();
        let i1 = Intersection::new(5.0, s);
        let i2 = Intersection::new(7.0, s);
        let i3 = Intersection::new(-3.0, s);
        let i4 = Intersection::new(2.0, s);
        let xs = intersections!(i1, i2, i3, i4);

        let i = Intersection::hit(xs).unwrap();
        assert_eq!(i, i4);
    }
}
