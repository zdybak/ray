#![allow(dead_code)]
use crate::light::Light;
use crate::sphere::Sphere;
use crate::raytuple::RayTuple;
use crate::color::Color;
use crate::matrix::Matrix;

pub struct World {
    light: Light,
    objects: Vec<Sphere>,
}

impl World {
    //here we are skipping the book's empty world creation because it's a hassle to
    //initiate a struct without giving it's members values.  We can always refactor
    //and put the light in a Vec so that it can be implemented without any lights 
    //in the Vec if we need to, but I don't see that so far.

    pub fn default_world() -> Self {
        let mut s1 = Sphere::new();
        s1.material.color = Color::new(0.8,1.0,0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;

        let mut s2 = Sphere::new();
        s2.set_transform(Matrix::scaling(0.5,0.5,0.5));

        Self {
            light: Light::point_light(RayTuple::point(-10.0,10.0,-10.0)
                , Color::new(1.0,1.0,1.0)),
            objects: vec![s1,s2],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //I've done some modifications to this test, since we are using UUID's in sphere initialization,
    //I only test to make sure the objects contain the non-default characteristics.
    //Another option would to be to remove the uuid from the Sphere's PartialEQ implementation
    #[test]
    fn create_default_world() {
        let l = Light::point_light(RayTuple::point(-10.0,10.0,-10.0), Color::new(1.0,1.0,1.0));
        let mut s1 = Sphere::new();
        s1.material.color = Color::new(0.8,1.0,0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;
        let mut s2 = Sphere::new();
        s2.set_transform(Matrix::scaling(0.5,0.5,0.5));
        let w = World::default_world();

        assert_eq!(w.light, l);
        assert_eq!(w.objects[0].material, s1.material);
        assert_eq!(w.objects[1].get_transform(), s2.get_transform());
    }
}