#![allow(dead_code)]
mod canvas;
mod color;
mod matrix;
mod point_light;
mod ray;
mod sphere;
mod tuple;

const EPSILON: f64 = 0.00001;
const PI: f64 = std::f64::consts::PI;

use canvas::Canvas;
use canvas::ToPPM;
use color::Color;
use point_light::PointLight;
use ray::Ray;
use sphere::Material;
use sphere::Sphere;
use tuple::Point;

fn main() {
    let width: usize = 800;
    let mut world = Canvas::new(width, width);
    let mut s = Sphere::default();
    match s.material {
        Material::Phong(ref mut p) => p.color = Color::new(1.0, 0.2, 1.0),
    }
    let cw: usize = width;
    let size = 10f64;
    let half = size / 2.0;
    for j in 0..cw {
        for i in 0..cw {
            let x = -(half as f64) + (i as f64 / cw as f64) * size;
            let y = (half as f64) - (j as f64 / cw as f64) * size;
            let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
            let point = Point::new(x, y, 10.0);
            let ray = Ray::new(
                Point::new(0.0, 0.0, -5.0),
                (point - Point::new(0.0, 0.0, -5.0)).normalize(),
            );

            let xs = s.intersect(ray);

            if let Some(hit) = xs.hit() {
                let intersection_point = ray.position(hit.t);
                let normal = hit.object.normal_at(intersection_point);
                let eye = -(ray.direction());
                let color = hit.object.material.lighting(light, point, eye, normal);

                world.set_color_at_pixel(i as f64, j as f64, color);
            }
        }
    }
    world.save_as_ppm("out.ppm".to_string()).unwrap();
}
