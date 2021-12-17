#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
mod canvas;
mod color;
mod matrix;
mod ray;
mod sphere;
mod tuple;
use crate::{
    matrix::Scaling,
    sphere::{Intersection, Intersections},
    tuple::Tuple,
};
use canvas::Canvas;
use color::Color;
use matrix::Translation;
use ray::Ray;
use sphere::Sphere;
// use matrix::{Rotation, Scaling, Translation};
use std::{f64::consts::PI, time::Instant};
use tuple::{Point, Vector};
fn main() {
    let mut world = Canvas::new(800, 800);
    // let mut center = Point::new(400, 400, 0);
    let s = Sphere::default();
    let cw: usize = 800;
    let size = 10f64;
    let half = size / 2.0;
    for j in 0..cw {
        for i in 0..cw {
            let x = -(half as f64) + (i as f64 / cw as f64) * size;
            let y = (half as f64) - (j as f64 / cw as f64) * size;

            let point = Point::new(x, y, 10.0);
            let ray = Ray::new(
                Point::new(0.0, 0.0, -5.0),
                (point - Point::new(0.0, 0.0, -5.0)).normalize(),
            );

            let xs = s.intersect(ray);

            if let Some(_i) = xs.hit() {
                world.set_color_at_pixel(i as f64, j as f64, Color::new(1.0, 0.0, 0.0));
            }
        }
    }
    world.save_as_ppm("out.ppm".to_string()).unwrap();
}
