#![allow(dead_code)]

use crate::color::Color;
use crate::raytuple::RayTuple;

#[derive(Debug)]
pub struct Light {
    pub position: RayTuple,
    pub intensity: Color,
}

impl Light {
    pub fn point_light(position: RayTuple, intensity: Color) -> Self {
        Self {
            position,
            intensity,
        }
    }
}

impl PartialEq for Light {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position && self.intensity == other.intensity
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_light_has_position_and_intensity() {
        let intensity = Color::new(1.0, 1.0, 1.0);
        let position = RayTuple::point(0.0, 0.0, 0.0);
        let light = Light::point_light(position, intensity);

        assert_eq!(light.intensity, intensity);
        assert_eq!(light.position, position);
    }
}
