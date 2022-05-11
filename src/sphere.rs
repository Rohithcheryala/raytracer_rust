use super::ray::Ray;
use crate::{
    intersections::{Intersection, Intersections},
    material::{Material, Phong},
    matrix::Matrix,
    tuple::Tuple,
};

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub transform: Matrix<4>,
    pub material: Material,
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

    /// ```
    /// use raytracer_rust::sphere::Sphere;
    /// use raytracer_rust::tuple::Tuple;
    /// use raytracer_rust::ray::Ray;
    /// let ray = Ray::new(Tuple::Point(0.0, 0.0, -5.0), Tuple::Vector(0.0, 0.0, 1.0));
    /// let sphere = Sphere::default();
    /// let xs = sphere.intersect(&ray);
    /// assert_eq!(xs.count(), 2);
    /// assert_eq!(xs[0].t, 4.0);
    /// assert_eq!(xs[1].t, 6.0);
    /// ```
    pub fn intersect(&self, ray: &Ray) -> Intersections {
        let r = ray.transform(self.transform.inverse());
        let sphere_to_ray = r.origin() - Tuple::Point(0.0, 0.0, 0.0);
        // origin: (Ox, Oy,Oz)
        // let (ox, oy, oz, _ow) = r.origin.as_tuple();
        // direction: (dx,dy,dz)
        // let (dx, dy, dz, _dw) = r.direction.as_tuple();
        // any point on ray: ( Ox+(dx*t), Oy+(dy*t), Oz+(dz*t) )
        // if distance of this point from sphere center is r then
        // this point lies on the sphere
        // eqn: x^2 + y^2 + z^2 = r^2 = 1^2
        // substituting and rearranging the terms gives
        // (dx^2+dy^2+dz^2)t^2 + 2(oxdx+oydy+ozdz)t + (ox^2+oy^2+oz^2-r^2) - 1 = 0
        let a = r.direction().dot(&r.direction()); // dx.powi(2) + dy.powi(2) + dz.powi(2);
        let b = 2.0 * r.direction().dot(&sphere_to_ray); // 2.0 * (ox * dx + oy * dy + oz * dz);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0; // ox.powi(2) + oy.powi(2) + oz.powi(2) - (self.r as f64).powi(2);

        let disc = b * b - 4.0 * a * c;
        let d1 = (-b - disc.sqrt()) / (2.0 * a);
        let d2 = (-b + disc.sqrt()) / (2.0 * a);
        let mut i = Intersections::default();
        if !d1.is_nan() {
            i.insert(Intersection::new(d1, *self, *ray));
        }
        if !d2.is_nan() {
            i.insert(Intersection::new(d2, *self, *ray));
        }
        i
    }

    /// ```
    /// use raytracer_rust::sphere::Sphere;
    /// use raytracer_rust::tuple::Tuple;
    /// use raytracer_rust::matrix::Matrix;
    /// let sphere = Sphere::default();
    /// let n = sphere.normal_at(Tuple::Point(1.0, 0.0, 0.0));
    /// assert_eq!(n, Tuple::Vector(1.0, 0.0, 0.0));
    ///
    /// let sphere = Sphere::default().with_transform(Matrix::Translation(0.0, 1.0, 0.0));
    /// let n = sphere.normal_at(Tuple::Point(0.0, 1.70711, -0.70711));
    /// assert_eq!(n, Tuple::Vector(0.0, 0.70711, -0.70711));
    /// ```
    pub fn normal_at(&self, world_point: Tuple) -> Tuple {
        let object_point = self.transform.inverse() * world_point;
        let object_normal = object_point - Tuple::Point(0.0, 0.0, 0.0);
        let mut world_normal = self.transform.inverse().transpose() * object_normal;
        world_normal.w = 0.0;
        world_normal.normalize()
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
