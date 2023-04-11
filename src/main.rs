use canvas::Canvas;
use color::Color;

mod canvas;
mod color;
mod raytuple;

fn main() {
    let mut c = Canvas::new(10, 2);
    let color = Color::new(1.0, 0.8, 0.6);
    for x in 0..c.width {
        for y in 0..c.height {
            c.write_pixel(x, y, color);
        }
    }
    let ppm = c.to_ppm();
    for line in ppm.lines() {
        print!("{line}\n");
    }
}
