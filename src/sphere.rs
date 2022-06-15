use super::ray::Ray;
use crate::{
    body::{Intersectable, Body, IntoBody},
    material::{Material, Phong},
    matrix::Matrix,
    tuple::Tuple,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sphere {
    transform: Matrix<4>,
    material: Material,
}

impl Sphere {
    pub fn new(transform: Matrix<4>, material: Material) -> Self {
        Self {
            transform,
            material,
        }
    }

    pub fn with_transform(mut self, t: Matrix<4>) -> Self {
        self.transform = t;
        self
    }

    pub fn with_material(mut self, m: Material) -> Self {
        self.material = m;
        self
    }
}

impl Intersectable for Sphere {
    fn material(&self) -> &Material {
        &self.material
    }

    fn material_mut(&mut self) -> &mut Material {
        &mut self.material
    }

    fn transform(&self) -> &Matrix<4> {
        &self.transform
    }

    fn intersect_in_object_space(&self, object_space_ray: &Ray) -> Vec<f64> {
        let sphere_to_ray = object_space_ray.origin - Tuple::Point(0.0, 0.0, 0.0);

        let a = object_space_ray.direction.dot(&object_space_ray.direction);
        let b = 2.0 * object_space_ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;

        let disc = b * b - 4.0 * a * c;
        let d1 = (-b - disc.sqrt()) / (2.0 * a);
        let d2 = (-b + disc.sqrt()) / (2.0 * a);

        let mut ts = vec![];
        if !d1.is_nan() {
            ts.push(d1);
        }
        if !d2.is_nan() {
            ts.push(d2);
        }
        ts
    }

    fn normal_at_in_object_space(&self, object_point: Tuple) -> Tuple {
        (object_point - Tuple::Point(0.0, 0.0, 0.0)).normalize()
    }
}

impl From<Sphere> for Body {
    fn from(s: Sphere) -> Self {
        Body::Sphere(s)
    }
}

impl From<&Sphere> for Body {
    fn from(s: &Sphere) -> Self {
        Body::Sphere(*s)
    }
}

impl IntoBody for Sphere {
    fn into_body(&self) -> Body {
        Body::Sphere(*self)
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            transform: Matrix::Identity(),
            material: Material::Phong(Phong::default()),
        }
    }
}
