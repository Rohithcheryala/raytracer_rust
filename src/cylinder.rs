use crate::{
    body::{Intersectable, Body, IntoBody}, consts::EPSILON, material::Material, matrix::Matrix, ray::Ray,
    tuple::Tuple,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cylinder {
    pub transform: Matrix<4>,
    pub material: Material,
    pub height: f64,
    pub is_closed: bool,
}

impl Cylinder {
    pub fn new(transform: Matrix<4>, material: Material, height: f64, is_closed: bool) -> Self {
        Self {
            transform,
            material,
            height,
            is_closed,
        }
    }
}

impl Intersectable for Cylinder {
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
        fn check_cap(ray: &Ray, t: f64) -> bool {
            let x = ray.origin.x + ray.direction.x * t;
            let z = ray.origin.z + ray.direction.z * t;
            x.powi(2) + z.powi(2) <= 1.0
        }
        fn intersect_caps(cyl: &Cylinder, ray: &Ray) -> Vec<f64> {
            if !cyl.is_closed || ray.direction.y.abs() < EPSILON {
                return vec![];
            }
            let min = -cyl.height / 2.0;
            let max = cyl.height / 2.0;

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
        let a = object_space_ray.direction.x.powi(2) + object_space_ray.direction.z.powi(2);
        let b = 2.0
            * (object_space_ray.direction.x * object_space_ray.origin.x
                + object_space_ray.direction.z * object_space_ray.origin.z);
        let c = object_space_ray.origin.x.powi(2) + object_space_ray.origin.z.powi(2) - 1.0;

        let disc = b.powi(2) - 4.0 * a * c;
        let mut d1 = (-b - disc.sqrt()) / (2.0 * a);
        let mut d2 = (-b + disc.sqrt()) / (2.0 * a);

        if d1 > d2 {
            std::mem::swap(&mut d1, &mut d2);
        }

        let mut res = vec![];

        let y1 = object_space_ray.origin.y + d1 * object_space_ray.direction.y;
        if y1 > -self.height / 2.0 && y1 < self.height / 2.0 && !d1.is_nan() {
            res.push(d1);
        }

        let y2 = object_space_ray.origin.y + d2 * object_space_ray.direction.y;
        if y2 > -self.height / 2.0 && y2 < self.height / 2.0 && !d2.is_nan() {
            res.push(d2);
        }

        // check for caps intersections
        let cap_res = intersect_caps(self, object_space_ray);
        res.extend(cap_res.into_iter());
        res.sort_by(|a, b| a.partial_cmp(b).unwrap());
        // FIXME: Why needed to filter-out others and take only 2
        res.into_iter().take(2).collect()
    }

    fn normal_at_in_object_space(&self, object_space_point: Tuple) -> Tuple {
        let dist = object_space_point.x.powi(2) + object_space_point.z.powi(2);
        let y = object_space_point.y;
        if dist < 1.0 && y <= -self.height / 2.0 + EPSILON {
            Tuple::Vector(0.0, -1.0, 0.0)
        } else if dist < 1.0 && y >= self.height / 2.0 - EPSILON {
            Tuple::Vector(0.0, 1.0, 0.0)
        } else {
            Tuple::Vector(object_space_point.x, 0.0, object_space_point.z)
        }
    }
}

impl From<Cylinder> for Body {
    fn from(c: Cylinder) -> Self {
        Body::Cylinder(c)
    }
}

impl From<&Cylinder> for Body {
    fn from(c: &Cylinder) -> Self {
        Body::Cylinder(*c)
    }
}

impl IntoBody for Cylinder {
    fn into_body(&self) -> Body {
        Body::Cylinder(*self)
    }
}

#[cfg(test)]
mod tests {
    use crate::material::Phong;

    use super::*;
    #[test]
    fn caps_works() {
        let cyl = Cylinder::new(
            Matrix::Identity(),
            Material::Phong(Phong {
                ..Default::default()
            }),
            2.0,
            true,
        );
        let ray = Ray::new(Tuple::Point(-1.0, -2.0, 0.0), Tuple::Vector(1.0, 1.0, 0.0));
        dbg!(cyl.normal_at_in_object_space(Tuple::Point(0.0, -2.0, 0.0)));
        let res = cyl.intersect(&ray);

        dbg!(res.hit());
    }
}
