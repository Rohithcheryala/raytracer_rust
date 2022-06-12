use crate::{
    body::Intersectable, consts::EPSILON, material::Material, matrix::Matrix, max, min, ray::Ray,
    tuple::Tuple,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cube {
    transform: Matrix<4>,
    material: Material,
}

impl Cube {
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

impl Intersectable for Cube {
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
        fn check_axis(origin: f64, direction: f64) -> (f64, f64) {
            let tmin_numerator = -1.0 - origin;
            let tmax_numerator = 1.0 - origin;
            let (mut tmin, mut tmax);
            if direction.abs() >= EPSILON {
                tmin = tmin_numerator / direction;
                tmax = tmax_numerator / direction;
            } else {
                tmin = tmin_numerator * f64::INFINITY;
                tmax = tmax_numerator * f64::INFINITY;
            }

            if tmin > tmax {
                std::mem::swap(&mut tmin, &mut tmax);
            }
            (tmin, tmax)
        }

        let (xtmin, xtmax) = check_axis(object_space_ray.origin.x, object_space_ray.direction.x);
        let (ytmin, ytmax) = check_axis(object_space_ray.origin.y, object_space_ray.direction.y);
        let (ztmin, ztmax) = check_axis(object_space_ray.origin.z, object_space_ray.direction.z);

        let tmin = max!(xtmin, ytmin, ztmin);
        let tmax = min!(xtmax, ytmax, ztmax);

        if tmin > tmax {
            vec![]
        } else {
            vec![tmin, tmax]
        }
    }

    fn normal_at_in_object_space(&self, object_space_point: Tuple) -> Tuple {
        let max = max!(
            object_space_point.x.abs(),
            object_space_point.y.abs(),
            object_space_point.z.abs()
        );
        if max == object_space_point.x.abs() {
            Tuple::Vector(object_space_point.x, 0.0, 0.0)
        } else if max == object_space_point.y.abs() {
            Tuple::Vector(0.0, object_space_point.y, 0.0)
        } else if max == object_space_point.z.abs() {
            Tuple::Vector(0.0, 0.0, object_space_point.z)
        } else {
            panic!("What the shit")
        }
    }
}
