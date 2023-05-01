#![allow(dead_code)]

use crate::color::Color;
use crate::light::Light;
use crate::raytuple::RayTuple;

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Material {
    pub fn new() -> Self {
        Self {
            color: Color::new(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }

    pub fn lighting(
        self,
        light: &Light,
        point: RayTuple,
        eyev: RayTuple,
        normalv: RayTuple,
    ) -> Color {
        //combine the surface color with the light's color/intensity
        let effective_color = self.color * light.intensity;

        //find the direction to the light source
        let lightv = (light.position - point).normalize();

        //compute the ambient contribution
        let ambient = effective_color * self.ambient;

        //light_dot_normal represents the cosine of the angle between the
        //light vector and the normal vector. A negative number means the
        //light is on the other side of the surface.
        let light_dot_normal = lightv.dot(normalv);

        if light_dot_normal < 0.0 {
            //diffuse and specular are black, so no need to even add or return them
            //we can simply return the ambient.
            return ambient;
        } else {
            //compute the diffuse contribution
            let diffuse = effective_color * self.diffuse * light_dot_normal;

            //reflection_dot_eye represents the cosine of the angle between the
            //reflection vector and the eye vector. A negative number means the
            //light reflects away from the eye.
            let reflectv = -lightv.reflect(normalv);
            let reflect_dot_eye = reflectv.dot(eyev);

            if reflect_dot_eye <= 0.0 {
                //specular is black, so return just the ambient + diffuse contributions
                return ambient + diffuse;
            } else {
                //compute the specular contribution
                let factor = f64::powf(reflect_dot_eye, self.shininess);
                let specular = light.intensity * self.specular * factor;

                ambient + diffuse + specular
            }
        }
    }
}

impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color
            && self.ambient == other.ambient
            && self.diffuse == other.diffuse
            && self.specular == other.specular
            && self.shininess == other.shininess
    }
}

#[cfg(test)]
mod tests {
    use crate::{light::Light, raytuple::RayTuple};

    use super::*;

    #[test]
    fn create_material() {
        let m = Material::new();

        assert_eq!(m.color, Color::new(1.0, 1.0, 1.0));
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.0);
    }

    #[test]
    fn lighting_eye_between() {
        //always given for lighting tests
        let m = Material::new();
        let position = RayTuple::point(0.0, 0.0, 0.0);

        let eyev = RayTuple::vector(0.0, 0.0, -1.0);
        let normalv = RayTuple::vector(0.0, 0.0, -1.0);
        let light = Light::point_light(RayTuple::point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));

        let result = m.lighting(&light, position, eyev, normalv);
        assert_eq!(result, Color::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn lighting_eye_between_offset_45() {
        //always given for lighting tests
        let m = Material::new();
        let position = RayTuple::point(0.0, 0.0, 0.0);

        let eyev = RayTuple::vector(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
        let normalv = RayTuple::vector(0.0, 0.0, -1.0);
        let light = Light::point_light(RayTuple::point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));

        let result = m.lighting(&light, position, eyev, normalv);
        assert_eq!(result, Color::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn lighting_light_offset_45() {
        //always given for lighting tests
        let m = Material::new();
        let position = RayTuple::point(0.0, 0.0, 0.0);

        let eyev = RayTuple::vector(0.0, 0.0, -1.0);
        let normalv = RayTuple::vector(0.0, 0.0, -1.0);
        let light =
            Light::point_light(RayTuple::point(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

        let result = m.lighting(&light, position, eyev, normalv);
        assert_eq!(result, Color::new(0.7364, 0.7364, 0.7364));
    }

    #[test]
    fn lighting_eye_in_reflection() {
        //always given for lighting tests
        let m = Material::new();
        let position = RayTuple::point(0.0, 0.0, 0.0);

        let eyev = RayTuple::vector(0.0, -2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
        let normalv = RayTuple::vector(0.0, 0.0, -1.0);
        let light =
            Light::point_light(RayTuple::point(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

        let result = m.lighting(&light, position, eyev, normalv);
        assert_eq!(result, Color::new(1.6364, 1.6364, 1.6364));
    }

    #[test]
    fn lighting_light_behind_surface() {
        //always given for lighting tests
        let m = Material::new();
        let position = RayTuple::point(0.0, 0.0, 0.0);

        let eyev = RayTuple::vector(0.0, 0.0, -1.0);
        let normalv = RayTuple::vector(0.0, 0.0, -1.0);
        let light = Light::point_light(RayTuple::point(0.0, 0.0, 10.0), Color::new(1.0, 1.0, 1.0));

        let result = m.lighting(&light, position, eyev, normalv);
        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }
}
