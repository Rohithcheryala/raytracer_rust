use crate::{
    body::{Body, Intersectable, IntoBody},
    consts::EPSILON,
    material::{Material, Phong},
    matrix::Matrix,
    ray::Ray,
    tuple::Tuple,
};

#[derive(Clone, Debug)]
pub struct Plane {
    transform: Matrix<4>,
    material: Material,
}

impl PartialEq for Plane {
    fn eq(&self, other: &Self) -> bool {
        self.transform == other.transform && self.material == other.material
    }
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

    fn transform(&self) -> Matrix<4_usize> {
        self.transform
    }

    fn transform_mut(self: &mut Plane) -> &mut Matrix<4_usize> {
        &mut self.transform
    }

    fn intersect_in_object_space(&self, object_space_ray: &Ray) -> Vec<f64> {
        if object_space_ray.direction.y.abs() < EPSILON {
            vec![]
        } else {
            vec![-object_space_ray.origin.y / object_space_ray.direction.y]
        }
    }

    fn normal_at_in_object_space(&self, _object_space_point: Tuple) -> Tuple {
        Tuple::Vector(0.0, 1.0, 0.0)
    }
}

impl From<Plane> for Body {
    fn from(p: Plane) -> Self {
        Body::Plane(p)
    }
}

// impl From<&Plane> for Body {
//     fn from(p: &Plane) -> Self {
//         Body::Plane(*p)
//     }
// }

impl IntoBody for Plane {
    fn into_body(&self) -> Body {
        Body::Plane(self.clone())
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
