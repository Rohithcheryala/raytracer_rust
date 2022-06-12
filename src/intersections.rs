use crate::{
    body::{Body, Intersectable},
    computed_intersection::ComputedIntersection,
    material::Refractive,
    ray::Ray,
};
use std::ops::Index;

#[derive(Debug, Clone, Copy)]
pub struct Intersection {
    pub t: f64,
    pub body: Body,
    pub ray: Ray,
}

#[derive(Debug, Default, Clone)]
pub struct Intersections {
    // count: u8,
    data: Vec<Intersection>,
}

impl Intersection {
    pub fn new(t: f64, body: Body, ray: Ray) -> Self {
        Self { t, body, ray }
    }

    pub fn to_computed(self, mu_from: f32, mu_to: f32) -> ComputedIntersection {
        let position = self.ray.position(self.t);
        let mut normalv = self.body.normal_at(position);
        let eyev = -self.ray.direction;
        let inside = normalv.dot(&eyev) < 0.0;
        if inside {
            normalv = -normalv;
        }
        let reflectv = self.ray.direction.reflect(normalv);
        ComputedIntersection::new(
            inside, position, self.body, eyev, normalv, reflectv, mu_from, mu_to,
        )
    }
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t
    }
}

impl Intersections {
    pub fn new(i: Vec<Intersection>) -> Self {
        Self { data: i }
    }

    pub fn insert(&mut self, i: Intersection) {
        self.data.push(i);
    }

    pub fn extend(&mut self, xs: Intersections) {
        self.data.extend(xs.data.into_iter());
    }

    pub fn count(&self) -> usize {
        self.data.len()
    }

    pub fn sort_by(&mut self, f: impl Fn(&Intersection, &Intersection) -> std::cmp::Ordering) {
        self.data.sort_by(f);
    }

    pub fn filter_nan(&mut self) {
        self.data.retain(|x| !x.t.is_nan());
    }

    pub fn iter(&self) -> std::slice::Iter<Intersection> {
        self.data.iter()
    }

    /// ```
    /// use raytracer_rust::sphere::Sphere;
    /// use raytracer_rust::tuple::Tuple;
    /// use raytracer_rust::ray::Ray;
    /// use raytracer_rust::intersections::{Intersection, Intersections};
    /// let i1 = Intersection::new(1.0, Sphere::default().into(), Ray::new(Tuple::Point(0.0, 0.0, -5.0), Tuple::Vector(0.0, 0.0, 1.0)));
    /// let i2 = Intersection::new(2.0, Sphere::default().into(), Ray::new(Tuple::Point(0.0, 0.0, -5.0), Tuple::Vector(0.0, 0.0, 1.0)));
    /// let xs = Intersections::new(vec![i1, i2]);
    /// assert_eq!(xs.hit(), Some(&i1));
    /// ```
    pub fn hit(&self) -> Option<&Intersection> {
        for intersection in self.data.iter() {
            if intersection.t > 0.0 {
                return Some(intersection);
            }
        }
        None
    }

    // FIXME: comeup with better algorithm.
    pub fn get_mu_shift(&self, intersection: &Intersection) -> (f32, f32) {
        let mut containers: Vec<Body> = vec![];
        let (mut mu_from, mut mu_to) = (1.0, 1.0);
        for i in self.data.iter() {
            if i == intersection && !containers.is_empty() {
                mu_from = containers.last().unwrap().material().refractive_index();
            }
            if containers.contains(&i.body) {
                let index = containers.iter().position(|x| x == &i.body).unwrap();
                containers.remove(index);
            } else {
                containers.push(i.body);
            }

            if i == intersection {
                if !containers.is_empty() {
                    mu_to = containers.last().unwrap().material().refractive_index();
                };
                break;
            }
        }
        (mu_from, mu_to)
    }
}

impl Index<usize> for Intersections {
    type Output = Intersection;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        body::Intersectable, material::Phong, matrix::Matrix, sphere::Sphere, tuple::Tuple,
    };

    #[test]
    fn mu_shift_works() {
        let s4 = Sphere::new(
            Matrix::Scaling(4.0, 4.0, 4.0),
            crate::material::Material::Phong(Phong {
                refractive_index: 1.5,
                ..Default::default()
            }),
        );
        let sdown = Sphere::new(
            Matrix::Translation(0.0, 0.0, -0.25),
            crate::material::Material::Phong(Phong {
                refractive_index: 2.0,
                ..Default::default()
            }),
        );
        let sup = Sphere::new(
            Matrix::Translation(0.0, 0.0, 0.25),
            crate::material::Material::Phong(Phong {
                refractive_index: 2.5,
                ..Default::default()
            }),
        );
        let r = Ray::new(Tuple::Point(0.0, 0.0, -5.0), Tuple::Vector(0.0, 0.0, 1.0));

        let xs4 = s4.intersect(&r);
        let xsdown = sdown.intersect(&r);
        let xsup = sup.intersect(&r);

        let mut xs = Intersections::new(vec![]);
        xs.extend(xs4);
        xs.extend(xsdown);
        xs.extend(xsup);

        xs.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());

        let (mu_from, mu_to) = xs.get_mu_shift(xs.hit().unwrap());

        assert_eq!(mu_from, 1.0);
        assert_eq!(mu_to, 1.5);
    }
}
