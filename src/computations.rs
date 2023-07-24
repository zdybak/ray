use crate::raytuple::RayTuple;
use crate::shape::Shape;

#[derive(Debug)]
pub struct Computations {
    pub t: f64,
    pub object: Shape,
    pub point: RayTuple,
    pub over_point: RayTuple,
    pub eyev: RayTuple,
    pub normalv: RayTuple,
    pub inside: bool,
    pub reflectv: RayTuple,
    pub n1: f64,
    pub n2: f64,
    pub under_point: RayTuple,
}

impl Computations {
    pub fn new(
        t: f64,
        object: Shape,
        point: RayTuple,
        over_point: RayTuple,
        eyev: RayTuple,
        normalv: RayTuple,
        inside: bool,
        reflectv: RayTuple,
        n1: f64,
        n2: f64,
        under_point: RayTuple,
    ) -> Self {
        Self {
            t,
            object,
            point,
            over_point,
            eyev,
            normalv,
            inside,
            reflectv,
            n1,
            n2,
            under_point,
        }
    }
}
