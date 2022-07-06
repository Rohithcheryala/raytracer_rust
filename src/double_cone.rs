use crate::{
    body::{Body, Intersectable, IntoBody},
    consts::EPSILON,
    material::Material,
    matrix::Matrix,
    ray::Ray,
    tuple::Tuple,
};

#[derive(Clone, Debug)]
pub struct DoubleCone {
    transform: Matrix<4>,
    material: Material,
    height: f64,
    is_closed: bool,
}

impl PartialEq for DoubleCone {
    fn eq(&self, other: &Self) -> bool {
        self.transform == other.transform
            && self.material == other.material
            && self.height == other.height
            && self.is_closed == other.is_closed
    }
}

impl DoubleCone {
    pub fn new(transform: Matrix<4>, material: Material, height: f64, is_closed: bool) -> Self {
        Self {
            transform,
            material,
            height,
            is_closed,
        }
    }
}

impl Intersectable for DoubleCone {
    fn material(&self) -> &Material {
        &self.material
    }

    fn material_mut(&mut self) -> &mut Material {
        &mut self.material
    }

    fn transform(&self) -> Matrix<4_usize> {
        self.transform
    }

    fn transform_mut(&mut self) -> &mut Matrix<4_usize> {
        &mut self.transform
    }

    fn intersect_in_object_space(&self, object_space_ray: &Ray) -> Vec<f64> {
        fn check_cap(ray: &Ray, t: f64) -> bool {
            let x = ray.origin.x + ray.direction.x * t;
            let y = ray.origin.y + ray.direction.y * t;
            let z = ray.origin.z + ray.direction.z * t;
            x.powi(2) + z.powi(2) <= y.powi(2)
        }
        fn intersect_caps(dcone: &DoubleCone, ray: &Ray) -> Vec<f64> {
            if !dcone.is_closed || ray.direction.y.abs() < EPSILON {
                return vec![];
            }
            let min = -dcone.height / 2.0;
            let max = dcone.height / 2.0;

            let tmin = (min - ray.origin.y) / ray.direction.y;
            let tmax = (max - ray.origin.y) / ray.direction.y;
            let mut res = vec![];
            if check_cap(ray, tmin) {
                res.push(tmin);
            }
            if check_cap(ray, tmax) {
                res.push(tmax);
            }
            res
        }
        let a = object_space_ray.direction.x.powi(2) - object_space_ray.direction.y.powi(2)
            + object_space_ray.direction.z.powi(2);
        let b = 2.0
            * (object_space_ray.origin.x * object_space_ray.direction.x
                - object_space_ray.origin.y * object_space_ray.direction.y
                + object_space_ray.origin.z * object_space_ray.direction.z);
        let c = object_space_ray.origin.x.powi(2) - object_space_ray.origin.y.powi(2)
            + object_space_ray.origin.z.powi(2);

        let mut res = vec![];
        if a.abs() < EPSILON {
            res.push(-c / (2.0 * b))
        } else {
            let disc = b.powi(2) - 4.0 * a * c;
            let mut d1 = (-b - disc.sqrt()) / (2.0 * a);
            let mut d2 = (-b + disc.sqrt()) / (2.0 * a);

            if d1 > d2 {
                std::mem::swap(&mut d1, &mut d2);
            }

            let y1 = object_space_ray.origin.y + d1 * object_space_ray.direction.y;
            if y1 > -self.height / 2.0 && y1 < self.height / 2.0 && !d1.is_nan() {
                res.push(d1);
            }

            let y2 = object_space_ray.origin.y + d2 * object_space_ray.direction.y;
            if y2 > -self.height / 2.0 && y2 < self.height / 2.0 && !d2.is_nan() {
                res.push(d2);
            }
        }
        // check for caps intersections
        let cap_res = intersect_caps(self, object_space_ray);
        res.extend(cap_res.into_iter());
        res.sort_by(|a, b| a.partial_cmp(b).unwrap());
        res
    }

    fn normal_at_in_object_space(&self, object_space_point: Tuple) -> Tuple {
        let dist = object_space_point.x.powi(2) + object_space_point.z.powi(2);
        let y = object_space_point.y;
        if dist < y.powi(2) && y <= -self.height / 2.0 + EPSILON {
            Tuple::Vector(0.0, -1.0, 0.0)
        } else if dist < y.powi(2) && y >= self.height / 2.0 - EPSILON {
            Tuple::Vector(0.0, 1.0, 0.0)
        } else {
            Tuple::Vector(object_space_point.x, -y, object_space_point.z).normalize()
        }
    }
}

impl From<DoubleCone> for Body {
    fn from(dc: DoubleCone) -> Self {
        Body::DoubleCone(dc)
    }
}

// impl From<&DoubleCone> for Body {
//     fn from(dc: &DoubleCone) -> Self {
//         Body::DoubleCone(*dc)
//     }
// }

impl IntoBody for DoubleCone {
    fn into_body(&self) -> Body {
        Body::DoubleCone(self.clone())
    }
}
