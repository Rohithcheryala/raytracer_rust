#![allow(dead_code)]
mod canvas;
mod color;
mod matrix;
mod ray;
mod sphere;
mod tuple;
use crate::{
    sphere::{Intersection, Intersections},
    tuple::Tuple,
};
use canvas::Canvas;
use color::Color;
use ray::Ray;
use sphere::Sphere;
// use matrix::{Rotation, Scaling, Translation};
use std::f64::consts::PI;
use tuple::{Point, Vector};
fn main() {
    let mut world = Canvas::new(800, 800);
    let mut center = Point::new(400, 400, 0);
    let s = Sphere::new(1);
    let r = Ray::new(Point::new(0, 0, 0), Vector::new(0, 0, 1));
    let xs = s.intersect(r);

    let i1 = Intersection {
        t: -1.0,
        object: &s,
    };
    let i2 = Intersection {
        t: -2.0,
        object: &s,
    };

    let s = Intersections::new(2, i1, i2);

    let xs = s.hit();
    println!("{:?}", xs);
    // world.save_as_ppm("out.ppm".to_string()).unwrap();
}
