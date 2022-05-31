use crate::{body::Body, computed_intersection::ComputedIntersection, consts::EPSILON, ray::Ray};
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

    pub fn to_computed(self) -> ComputedIntersection {
        let position = self.ray.position(self.t);
        let mut normalv = self.body.normal_at(position);
        let eyev = -self.ray.direction;
        let inside = normalv.dot(&eyev) < 0.0;
        if inside {
            normalv = -normalv;
        }
        let over_point = position + normalv * EPSILON;
        let reflectv = self.ray.direction.reflect(normalv);
        ComputedIntersection::new(
            inside, position, over_point, self.body, eyev, normalv, reflectv,
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
}

impl Index<usize> for Intersections {
    type Output = Intersection;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            n if n < self.count() => &self.data[n],
            _ => panic!("Index out of range."),
        }
    }
}
