use crate::{
    cube::Cube,
    cylinder::Cylinder,
    double_cone::DoubleCone,
    intersections::{Intersection, Intersections},
    material::Material,
    matrix::Matrix,
    plane::Plane,
    ray::Ray,
    sphere::Sphere,
    tuple::Tuple,
};

pub trait IntoBody {
    fn into_body(&self) -> Body;
}

pub trait Intersectable
where
    Self: IntoBody,
{
    fn material(&self) -> &Material;
    fn material_mut(&mut self) -> &mut Material;
    fn transform(&self) -> Matrix<4>;

    /// ```
    /// use raytracer_rust::sphere::Sphere;
    /// use raytracer_rust::tuple::Tuple;
    /// use raytracer_rust::ray::Ray;
    /// use raytracer_rust::body::Intersectable;
    /// let ray = Ray::new(Tuple::Point(0.0, 0.0, -5.0), Tuple::Vector(0.0, 0.0, 1.0));
    /// let sphere = Sphere::default();
    /// let xs = sphere.intersect(&ray);
    /// assert_eq!(xs.count(), 2);
    /// assert_eq!(xs[0].t, 4.0);
    /// assert_eq!(xs[1].t, 6.0);
    /// ```
    fn intersect(&self, ray: &Ray) -> Intersections {
        let object_space_ray = ray.transform(self.transform().inverse());
        let result = self.intersect_in_object_space(&object_space_ray);
        Intersections::new(
            result
                .into_iter()
                .map(|t| Intersection::new(t, self.into_body(), *ray))
                .collect(),
        )
    }
    /// ```
    /// use raytracer_rust::sphere::Sphere;
    /// use raytracer_rust::tuple::Tuple;
    /// use raytracer_rust::matrix::Matrix;
    /// use raytracer_rust::body::Intersectable;
    /// let sphere = Sphere::default();
    /// let n = sphere.normal_at(Tuple::Point(1.0, 0.0, 0.0));
    /// assert_eq!(n, Tuple::Vector(1.0, 0.0, 0.0));
    ///
    /// let sphere = Sphere::default().with_transform(Matrix::Translation(0.0, 1.0, 0.0));
    /// let n = sphere.normal_at(Tuple::Point(0.0, 1.70711, -0.70711));
    /// assert_eq!(n, Tuple::Vector(0.0, 0.70711, -0.70711));
    /// ```
    fn normal_at(&self, point: Tuple) -> Tuple {
        let object_point = self.transform().inverse() * point;
        let object_normal = self.normal_at_in_object_space(object_point);
        let mut world_normal = self.transform().inverse().transpose() * object_normal;
        world_normal.w = 0.0;
        world_normal.normalize()
    }

    /// Returns the ```sorted``` distances to the intersection points in a vector.
    fn intersect_in_object_space(&self, object_space_ray: &Ray) -> Vec<f64>;
    fn normal_at_in_object_space(&self, object_space_point: Tuple) -> Tuple;
}

#[derive(Clone, Debug, PartialEq)]
pub enum Body {
    Sphere(Sphere),
    Plane(Plane),
    Cube(Cube),
    Cylinder(Cylinder),
    DoubleCone(DoubleCone),
}

impl IntoBody for Body {
    fn into_body(&self) -> Body {
        self.clone()
    }
}

impl Intersectable for Body {
    fn material(&self) -> &Material {
        match self {
            Body::Sphere(s) => s.material(),
            Body::Plane(p) => p.material(),
            Body::Cube(c) => c.material(),
            Body::Cylinder(c) => c.material(),
            Body::DoubleCone(dc) => dc.material(),
        }
    }

    fn material_mut(&mut self) -> &mut Material {
        match self {
            Body::Sphere(s) => s.material_mut(),
            Body::Plane(p) => p.material_mut(),
            Body::Cube(c) => c.material_mut(),
            Body::Cylinder(c) => c.material_mut(),
            Body::DoubleCone(dc) => dc.material_mut(),
        }
    }

    fn transform(&self) -> Matrix<4> {
        match self {
            Body::Sphere(s) => s.transform(),
            Body::Plane(p) => p.transform(),
            Body::Cube(c) => c.transform(),
            Body::Cylinder(c) => c.transform(),
            Body::DoubleCone(dc) => dc.transform(),
        }
    }

    fn intersect_in_object_space(&self, ray: &Ray) -> Vec<f64> {
        match self {
            Body::Sphere(s) => s.intersect_in_object_space(ray),
            Body::Plane(p) => p.intersect_in_object_space(ray),
            Body::Cube(c) => c.intersect_in_object_space(ray),
            Body::Cylinder(c) => c.intersect_in_object_space(ray),
            Body::DoubleCone(dc) => dc.intersect_in_object_space(ray),
        }
    }

    fn normal_at_in_object_space(&self, point: Tuple) -> Tuple {
        match self {
            Body::Sphere(s) => s.normal_at_in_object_space(point),
            Body::Plane(p) => p.normal_at_in_object_space(point),
            Body::Cube(c) => c.normal_at_in_object_space(point),
            Body::Cylinder(c) => c.normal_at_in_object_space(point),
            Body::DoubleCone(dc) => dc.normal_at_in_object_space(point),
        }
    }
}
