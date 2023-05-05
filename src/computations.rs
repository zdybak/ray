use crate::raytuple::RayTuple;
use crate::sphere::Sphere;

pub struct Computations {
    pub t: f64,
    pub object: Sphere,
    pub point: RayTuple,
    pub eyev: RayTuple,
    pub normalv: RayTuple,
    pub inside: bool,
}

impl Computations {
    pub fn new(
        t: f64,
        object: Sphere,
        point: RayTuple,
        eyev: RayTuple,
        normalv: RayTuple,
        inside: bool,
    ) -> Self {
        Self {
            t,
            object,
            point,
            eyev,
            normalv,
            inside,
        }
    }
}
