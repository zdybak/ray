#![allow(dead_code)]
use crate::color::Color;
use std::mem;

#[derive(Debug)]
pub struct Canvas {
    width: i32,
    height: i32,
    pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: i32, height: i32) -> Self {
        let len = width * height;
        let mut pixels = Vec::new();
        for _ in 0..len {
            pixels.push(Color::new(0.0, 0.0, 0.0));
        }

        Self {
            width,
            height,
            pixels,
        }
    }

    pub fn pixel_at(&self, x: i32, y: i32) -> Color {
        //convert x,y coords to index
        let i = (y * self.width + x) as usize;
        *self.pixels.get(i).unwrap()
    }

    pub fn write_pixel(&mut self, x: i32, y: i32, c: Color) {
        let i = (y * self.width + x) as usize;
        let p = self.pixels.get_mut(i).unwrap();
        let _old_color = mem::replace(p, c);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_blank_canvas() {
        let c = Canvas::new(10, 20);

        for x in 0..10 {
            for y in 0..20 {
                assert_eq!(c.pixel_at(x, y), Color::new(0.0, 0.0, 0.0));
            }
        }
    }

    #[test]
    fn write_pixel_to_canvas() {
        let mut c = Canvas::new(10, 20);
        let red = Color::new(1.0, 0.0, 0.0);
        c.write_pixel(2, 3, red);
        assert_eq!(c.pixel_at(2, 3), red);
    }
}
