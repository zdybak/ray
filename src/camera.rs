#![allow(dead_code)]
use crate::matrix::Matrix;

#[derive(Debug)]
pub struct Camera {
    hsize: f64,
    vsize: f64,
    field_of_view: f64,
    transform: Matrix,
    pixel_size: f64,
    half_width: f64,
    half_height: f64,
}

impl Camera {
    pub fn new(hsize: f64, vsize: f64, field_of_view: f64) -> Self {
        let half_view = (field_of_view / 2.0).tan();
        let aspect = hsize / vsize;
        let half_width = match aspect >= 1.0 {
            true => half_view,
            false => half_view * aspect,
        };
        let half_height = match aspect >= 1.0 {
            true => half_view / aspect,
            false => half_view,
        };
        Self {
            hsize,
            vsize,
            field_of_view,
            transform: Matrix::identity(),
            pixel_size: (half_width * 2.0) / hsize,
            half_width,
            half_height,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::FRAC_PI_2;

    #[test]
    fn create_a_camera() {
        let hsize = 160.0;
        let vsize = 120.0;
        let fov = FRAC_PI_2;

        let c = Camera::new(hsize, vsize, fov);
        assert_eq!(c.hsize, 160.0);
        assert_eq!(c.vsize, 120.0);
        assert_eq!(c.field_of_view, FRAC_PI_2);
        assert_eq!(c.transform, Matrix::identity());
    }

    #[test]
    fn pixel_size_horizontal() {
        let c = Camera::new(200.0, 125.0, FRAC_PI_2);
        assert!((c.pixel_size - 0.01).abs() < f64::EPSILON);
    }

    #[test]
    fn pixel_size_vertical() {
        let c = Camera::new(125.0, 200.0, FRAC_PI_2);
        assert!((c.pixel_size - 0.01).abs() < f64::EPSILON);
    }
}
