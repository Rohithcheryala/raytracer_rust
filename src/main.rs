#![allow(dead_code)]
mod canvas;
mod color;
mod matrix;
mod tuple;
use canvas::Canvas;
use color::Color;
// use matrix::{Rotation, Scaling, Translation};
use std::f64::consts::PI;
use tuple::Point;
fn main() {
    let mut world = Canvas::new(800, 800);
    let center = Point::new(400, 400, 0);
    let color = Color::new(1.0, 1.0, 1.0);
    for i in 0..6 {
        let mut r = Point::new(200, 0, 0);
        let k = r
            .rotate_z(i as f64 * (2.0 * PI / 12.0))
            .translate_to_point(center);
        world.set_color_at_pixels(k.x, k.y, color);
    }
    let color = Color::new(1.0, 0.0, 0.0);
    for i in 6..12 {
        let mut r = Point::new(200, 0, 0);
        let k = r
            .rotate_z(i as f64 * (2.0 * PI / 12.0))
            .translate_to_point(center);
        world.set_color_at_pixels(k.x, k.y, color);
    }

    world.save_as_ppm("out.ppm".to_string()).unwrap();
}
