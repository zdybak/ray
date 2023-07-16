#![allow(dead_code)]

use crate::color::Color;
use crate::light::Light;
use crate::pattern::Pattern;
use crate::raytuple::RayTuple;
use crate::shape::Shape;

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
    pub pattern: Option<Pattern>,
    pub reflective: f64,
    pub transparency: f64,
    pub refractive_index: f64,
}

impl Material {
    pub fn new() -> Self {
        Self {
            color: Color::new(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            pattern: None,
            reflective: 0.0,
            transparency: 0.0,
            refractive_index: 1.0,
        }
    }

    pub fn lighting(
        self,
        shape: Shape,
        light: &Light,
        point: RayTuple,
        eyev: RayTuple,
        normalv: RayTuple,
        in_shadow: bool,
    ) -> Color {
        let pattern_color = match self.pattern {
            Some(p) => p.pattern_at_shape(shape, point),
            None => self.color,
        };

        //combine the surface color with the light's color/intensity
        let effective_color = pattern_color * light.intensity;

        //find the direction to the light source
        let lightv = (light.position - point).normalize();

        //compute the ambient contribution
        let ambient = effective_color * self.ambient;

        //light_dot_normal represents the cosine of the angle between the
        //light vector and the normal vector. A negative number means the
        //light is on the other side of the surface.
        let light_dot_normal = lightv.dot(normalv);

        if light_dot_normal < 0.0 || in_shadow {
            //diffuse and specular are black, so no need to even add or return them
            // OR if point is in shadow then we only use ambient
            //we can simply return the ambient
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

        let result = m.lighting(Shape::test_shape(), &light, position, eyev, normalv, false);
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

        let result = m.lighting(Shape::test_shape(), &light, position, eyev, normalv, false);
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

        let result = m.lighting(Shape::test_shape(), &light, position, eyev, normalv, false);
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

        let result = m.lighting(Shape::test_shape(), &light, position, eyev, normalv, false);
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

        let result = m.lighting(Shape::test_shape(), &light, position, eyev, normalv, false);
        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn lighting_with_surface_in_shadow() {
        let eyev = RayTuple::vector(0.0, 0.0, -1.0);
        let normalv = RayTuple::vector(0.0, 0.0, -1.0);
        let light = Light::point_light(RayTuple::point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let in_shadow = true;
        let m = Material::new();
        let position = RayTuple::point(0.0, 0.0, 0.0);

        let result = m.lighting(
            Shape::test_shape(),
            &light,
            position,
            eyev,
            normalv,
            in_shadow,
        );
        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn lighting_with_pattern_applied() {
        let mut m = Material::new();
        m.pattern = Some(Pattern::stripe_pattern(
            Color::new(1.0, 1.0, 1.0),
            Color::new(0.0, 0.0, 0.0),
        ));
        m.ambient = 1.0;
        m.diffuse = 0.0;
        m.specular = 0.0;
        let eyev = RayTuple::vector(0.0, 0.0, -1.0);
        let normalv = RayTuple::vector(0.0, 0.0, -1.0);
        let light = Light::point_light(RayTuple::point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let c1 = m.lighting(
            Shape::test_shape(),
            &light,
            RayTuple::point(0.9, 0.0, 0.0),
            eyev,
            normalv,
            false,
        );
        let c2 = m.lighting(
            Shape::test_shape(),
            &light,
            RayTuple::point(1.1, 0.0, 0.0),
            eyev,
            normalv,
            false,
        );

        assert_eq!(c1, Color::new(1.0, 1.0, 1.0));
        assert_eq!(c2, Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn reflectivity_for_default_material() {
        let m = Material::new();
        assert_eq!(m.reflective, 0.0);
    }

    #[test]
    fn default_values_for_refraction() {
        let m = Material::new();
        assert_eq!(m.transparency, 0.0);
        assert_eq!(m.refractive_index, 1.0);
    }
}
