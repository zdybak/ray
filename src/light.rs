#![allow(dead_code)]

use crate::raytuple::RayTuple;
use crate::color::Color;

#[derive(Debug)]
pub struct Light {
    intensity: Color,
    position: RayTuple,
}

impl Light {
    pub fn point_light(intensity: Color, position: RayTuple) -> Self {
        Self {
            intensity,
            position,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_light_has_position_and_intensity() {
        let intensity = Color::new(1.0,1.0,1.0);
        let position = RayTuple::point(0.0,0.0,0.0);
        let light = Light::point_light(intensity, position);

        assert_eq!(light.intensity, intensity);
        assert_eq!(light.position, position);
    }
}