#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
mod canvas;
mod color;
mod matrix;
mod point_light;
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
use point_light::PointLight;
use ray::Ray;
use sphere::{Material, Sphere};
// use matrix::{Rotation, Scaling, Translation};
use std::{f64::consts::PI, time::Instant};
use tuple::{Point, Vector};
fn main() {
    let width: usize = 800;
    let mut world = Canvas::new(width, width);
    let mut s = Sphere::default();
    s.material.color = Color::new(1.0, 0.2, 1.0);
    let cw: usize = width;
    let size = 10f64;
    let half = size / 2.0;
    for j in 0..cw {
        for i in 0..cw {
            let x = -(half as f64) + (i as f64 / cw as f64) * size;
            let y = (half as f64) - (j as f64 / cw as f64) * size;
            let light = PointLight::new(Color::new(1.0, 1.0, 1.0), Point::new(-10.0, 10.0, -10.0));
            let point = Point::new(x, y, 10.0);
            let ray = Ray::new(
                Point::new(0.0, 0.0, -5.0),
                (point - Point::new(0.0, 0.0, -5.0)).normalize(),
            );
            let xs = s.intersect(ray);

            if let Some(hit) = xs.hit() {
                let intersection_point = ray.position(hit.t);
                let normal = hit.object.normal_at(intersection_point);
                let eye = -(ray.direction);
                let color = hit.object.material.lighting(light, point, eye, normal);

                world.set_color_at_pixel(i as f64, j as f64, color);
            }
        }
    }
    world.save_as_ppm("out.ppm".to_string()).unwrap();
}
