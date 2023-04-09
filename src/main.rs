use canvas::Canvas;
use color::Color;

mod canvas;
mod color;
mod raytuple;

fn main() {
    let mut c = Canvas::new(3, 3);
    c.write_pixel(1, 1, Color::new(1.0, 1.0, 1.0));

    println!("{:?}", c);
}
