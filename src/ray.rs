use crate::{matrix::Matrix, tuple::Tuple};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ray {
    pub origin: Tuple,    // basically Point
    pub direction: Tuple, // basically Vector
}

impl Ray {
    #[inline]
    pub fn new(origin: Tuple, direction: Tuple) -> Self {
        Self { origin, direction }
    }

    pub fn origin(&self) -> Tuple {
        self.origin
    }

    pub fn direction(&self) -> Tuple {
        self.direction
    }

    /// ```
    /// use raytracer_rust::ray::Ray;
    /// use raytracer_rust::tuple::Tuple;
    /// let ray = Ray::new(Tuple::Point(1.0, 2.0, 3.0), Tuple::Vector(4.0, 5.0, 6.0));
    /// assert_eq!(ray.position(1.0), Tuple::Point(5.0, 7.0, 9.0));
    /// ```
    pub fn position(&self, t: f64) -> Tuple {
        self.origin + self.direction * t
    }

    /// ```
    /// use raytracer_rust::ray::Ray;
    /// use raytracer_rust::tuple::Tuple;
    /// use raytracer_rust::matrix::Matrix;
    /// let ray = Ray::new(Tuple::Point(1.0, 2.0, 3.0), Tuple::Vector(0.0, 1.0, 0.0));
    /// let transform = Matrix::Translation(3.0, 4.0, 5.0);
    /// assert_eq!(ray.transform(transform), Ray::new(Tuple::Point(4.0, 6.0, 8.0), Tuple::Vector(0.0, 1.0, 0.0)));
    /// let transform = Matrix::Scaling(2.0, 3.0, 4.0);
    /// assert_eq!(ray.transform(transform), Ray::new(Tuple::Point(2.0, 6.0, 12.0), Tuple::Vector(0.0, 3.0, 0.0)));
    /// ```
    pub fn transform(&self, t: Matrix<4>) -> Self {
        Self {
            origin: t * self.origin,
            direction: t * self.direction,
        }
    }

    // pub fn as_tuple(&self) -> (Tuple, Tuple) {
    //     (self.origin, self.direction)
    // }
}
