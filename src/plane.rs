use crate::{
    body::Intersectable,
    material::{Material, Phong},
    matrix::Matrix,
    ray::Ray,
    tuple::Tuple,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Plane {
    transform: Matrix<4>,
    material: Material,
}

impl Plane {
    pub fn new(transform: Matrix<4>, material: Material) -> Self {
        Self {
            transform,
            material,
        }
    }

    pub fn with_material(mut self, material: Material) -> Self {
        self.material = material;
        self
    }
}

impl Intersectable for Plane {
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
        vec![-object_space_ray.origin.y / object_space_ray.direction.y]
    }

    fn normal_at_in_object_space(&self, _object_space_point: Tuple) -> Tuple {
        Tuple::Vector(0.0, 1.0, 0.0)
    }
}

impl Default for Plane {
    fn default() -> Self {
        Self {
            material: Phong::default().into(),
            transform: Matrix::Identity(),
        }
    }
}
