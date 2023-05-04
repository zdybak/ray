use crate::raytuple::RayTuple;
use crate::sphere::Sphere;

pub struct Computations {
    pub t: f64,
    pub object: Sphere,
    pub point: RayTuple,
    pub eyev: RayTuple,
    pub normalv: RayTuple,
}

impl Computations {
    pub fn new(t: f64, object: Sphere, point: RayTuple, eyev: RayTuple, normalv: RayTuple) -> Self {
        Self {
            t,
            object,
            point,
            eyev,
            normalv,
        }
    }
}
