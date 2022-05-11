use std::sync::Mutex;

use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{canvas::Canvas, matrix::Matrix, ray::Ray, tuple::Tuple, world::World};

pub struct Camera {
    pub transform: Matrix<4>,
    pub hsize: usize,
    pub vsize: usize,
    pub field_of_view: f64,
    half_width: f64,
    half_height: f64,
    pixel_size: f64,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: f64) -> Self {
        let aspect = hsize as f64 / vsize as f64;
        let half_view = (field_of_view / 2.0).tan();
        let (half_width, half_height) = if aspect >= 1.0 {
            (half_view, half_view / aspect)
        } else {
            (half_view * aspect, half_view)
        };
        let pixel_size = (2.0 * half_width) / hsize as f64;
        Self {
            hsize,
            vsize,
            field_of_view,
            transform: Matrix::Identity(),
            half_width,
            half_height,
            pixel_size,
        }
    }

    /// ```
    /// use raytracer_rust::camera::Camera;
    /// use raytracer_rust::tuple::Tuple;
    /// use raytracer_rust::consts::PI_BY_2;
    ///
    /// let camera = Camera::new(201, 101, PI_BY_2);
    /// let r = camera.ray_for_pixel(100, 50);
    /// assert_eq!(r.origin, Tuple::Point(0.0, 0.0, 0.0));
    /// assert_eq!(r.direction, Tuple::Vector(0.0, 0.0, -1.0));
    ///
    /// let r = camera.ray_for_pixel(0, 0);
    /// assert_eq!(r.origin, Tuple::Point(-0.0, 0.0, 0.0));
    /// assert_eq!(r.direction, Tuple::Vector(0.66519, 0.33259, -0.66851));
    /// ```
    pub fn ray_for_pixel(&self, px: usize, py: usize) -> Ray {
        let xoffset = (px as f64 + 0.5) * self.pixel_size;
        let yoffset = (py as f64 + 0.5) * self.pixel_size;
        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;
        let pixel = self.transform.inverse() * Tuple::Point(world_x, world_y, -1.0);
        let origin = self.transform.inverse() * Tuple::Point(0, 0, 0);
        let direction = (pixel - origin).normalize();
        Ray::new(origin, direction)
    }

    pub fn render(&self, world: &World) -> Canvas {
        let mut canvas = Canvas::new(self.hsize, self.vsize);
        for y in 0..self.vsize {
            for x in 0..self.hsize {
                let ray = self.ray_for_pixel(x, y);
                let color = world.color_at(ray);
                canvas.set_color_at_pixel(x, y, color);
            }
        }
        canvas
    }

    pub fn render_par(&self, world: &World) -> Canvas {
        let canvas = Mutex::new(Canvas::new(self.hsize, self.vsize));
        (0..self.vsize).into_par_iter().for_each(|y| {
            (0..self.hsize).into_par_iter().for_each(|x| {
                let ray = self.ray_for_pixel(x, y);
                let color = world.color_at(ray);
                canvas.lock().unwrap().set_color_at_pixel(x, y, color);
            })
        });
        canvas.into_inner().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn camera_ray_for_pixel() {
        let c = Camera::new(201, 101, std::f64::consts::PI / 2.0);
        let ray = c.ray_for_pixel(0, 0);
        assert_eq!(ray.origin(), Tuple::Point(0.0, 0.0, 0.0));
        assert_eq!(ray.direction(), Tuple::Vector(0.66519, 0.33259, -0.66851));
    }

    #[test]
    fn render() {
        let world = World::default_from_book();
        let mut camera = Camera::new(11, 11, crate::consts::PI_BY_2);
        let from = Tuple::Point(0, 0, -5);
        let to = Tuple::Point(0, 0, 0);
        let up = Tuple::Vector(0, 1, 0);
        camera.transform = Matrix::view_transform(from, to, up);
        let image = camera.render(&world);
        assert_eq!(
            image.color_at_pixel(5, 5),
            crate::color::Color::new(0.38066, 0.47583, 0.2855)
        );
    }
}
