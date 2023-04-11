#![allow(dead_code)]
use crate::color::Color;
use std::mem;

#[derive(Debug)]
pub struct Canvas {
    pub width: i32,
    pub height: i32,
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

    pub fn to_ppm(&self) -> String {
        let h1 = String::from("P3\n");
        let h2 = format!("{} {}\n", self.width, self.height);
        let h3 = String::from("255\n");

        let mut pixel_data = String::new();

        let mut current_line = String::new();

        for y in 0..self.height {
            for x in 0..self.width {
                let this_pixel = self.pixel_at(x, y);
                let red = ((this_pixel.red * 255.0).round() as i32).clamp(0, 255);
                let green = ((this_pixel.green * 255.0).round() as i32).clamp(0, 255);
                let blue = ((this_pixel.blue * 255.0).round() as i32).clamp(0, 255);
                let r_str = format!("{}", red);
                let g_str = format!("{}", green);
                let b_str = format!("{}", blue);

                for slice in [r_str, g_str, b_str] {
                    if current_line.len() + slice.len() > 68 {
                        current_line += "\n";
                        pixel_data += &current_line;
                        current_line = String::new();
                    } else if current_line.len() > 0 {
                        current_line += &String::from(" ");
                    }
                    current_line += &slice;
                }
            }
            current_line += "\n";
            pixel_data += &current_line;
            current_line = String::new();
        }

        h1 + &h2 + &h3 + &pixel_data
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

    #[test]
    fn write_to_ppm() {
        let mut c = Canvas::new(5, 3);
        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0.0, 1.0);
        c.write_pixel(0, 0, c1);
        c.write_pixel(2, 1, c2);
        c.write_pixel(4, 2, c3);
        assert_eq!(c.to_ppm(),String::from("P3\n5 3\n255\n255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 128 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 0 0 0 0 0 0 0 255\n"));
    }

    #[test]
    fn no_line_more_than_70() {
        let mut c = Canvas::new(10, 2);
        let color = Color::new(1.0, 0.8, 0.6);
        for x in 0..c.width {
            for y in 0..c.height {
                c.write_pixel(x, y, color);
            }
        }
        let ppm = c.to_ppm();
        let mut line_num = 0;
        for line in ppm.lines() {
            line_num += 1;
            if line_num == 4 || line_num == 6 {
                assert_eq!(
                    line,
                    "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204"
                );
            }
            if line_num == 5 || line_num == 7 {
                assert_eq!(line, "153 255 204 153 255 204 153 255 204 153 255 204 153");
            }
        }
    }

    #[test]
    fn ppm_ends_with_new_line() {
        let c = Canvas::new(5, 3);
        let ppm = c.to_ppm();
        let len = ppm.len();
        assert_eq!(&ppm[len - 1..len], "\n");
    }
}
