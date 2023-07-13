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
        }
    }
}
