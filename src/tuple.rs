use crate::consts::EPSILON;
use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign},
};

/// A tuple of 4 floating point numbers.
/// This is used to represent a point or vector depending on 4th component in 3D space.
/// 4th component is w, which can be either 0 or 1.
/// w: 1 means the tuple is a point,
/// w: 0 means the tuple is a vector.
/// ```
/// use raytracer_rust::tuple::Tuple;
/// let point = Tuple { x: 1.0, y: 2.0, z: 3.0, w: 1.0 };
/// let vector = Tuple { x: 1.0, y: 2.0, z: 3.0, w: 0.0 };
/// ```
#[derive(Clone, Debug, Copy)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Tuple {
    #[inline]
    pub fn new<T: Into<f64>>(x: T, y: T, z: T, w: T) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            z: z.into(),
            w: w.into(),
        }
    }

    /// ```
    /// use raytracer_rust::tuple::Tuple;
    /// let point = Tuple::Point(1.0, 2.0, 3.0);
    /// assert!(point.is_point());
    /// ```
    #[inline]
    #[allow(non_snake_case)]
    pub fn Point<T: Into<f64>>(x: T, y: T, z: T) -> Tuple {
        Tuple {
            x: x.into(),
            y: y.into(),
            z: z.into(),
            w: 1.0,
        }
    }

    /// ```
    /// use raytracer_rust::tuple::Tuple;
    /// let vector = Tuple::Vector(1.0, 2.0, 3.0);
    /// assert!(vector.is_vector());
    /// ```
    #[inline]
    #[allow(non_snake_case)]
    pub fn Vector<T: Into<f64>>(x: T, y: T, z: T) -> Tuple {
        Tuple {
            x: x.into(),
            y: y.into(),
            z: z.into(),
            w: 0.0,
        }
    }

    /// ```
    /// use raytracer_rust::tuple::Tuple;
    /// let point = Tuple { x: 1.0, y: 2.0, z: 3.0, w: 1.0 };
    /// assert!(point.is_point());
    /// ```
    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    /// ```
    /// use raytracer_rust::tuple::Tuple;
    /// let vector = Tuple { x: 1.0, y: 2.0, z: 3.0, w: 0.0 };
    /// assert!(vector.is_vector());
    /// ```
    pub fn is_vector(&self) -> bool {
        !self.is_point()
    }

    /// ```
    /// use raytracer_rust::tuple::Tuple;
    /// let vector = Tuple::Vector(1.0, 2.0, 3.0);
    /// assert_eq!(vector.magnitude(), 3.7416573867739413);
    /// ```
    pub fn magnitude(&self) -> f64 {
        assert_eq!(self.w, 0.0); // magnitude only exists for vectors
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// ```
    /// use raytracer_rust::tuple::Tuple;
    /// let vector = Tuple::Vector(1.0, 2.0, 3.0);
    /// assert_eq!(vector.normalize(), Tuple::Vector(0.2672612419124244, 0.5345224838248488, 0.8017837257372732));
    /// ```
    pub fn normalize(&self) -> Self {
        let m = self.magnitude();
        Self {
            x: self.x / m,
            y: self.y / m,
            z: self.z / m,
            w: self.w / m,
        }
    }

    /// ```
    /// use raytracer_rust::tuple::Tuple;
    /// let vector = Tuple::Vector(1.0, 2.0, 3.0);
    /// assert_eq!(vector.dot(&vector), 14.0);
    /// ```
    pub fn dot(&self, other: &Self) -> f64 {
        // assert_eq!(self.w, 0.0); // magnitude only exists for vectors
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// ```
    /// use raytracer_rust::tuple::Tuple;
    /// let vector = Tuple::Vector(1.0, 2.0, 3.0);
    /// let vector2 = Tuple::Vector(2.0, 3.0, 4.0);
    /// assert_eq!(vector.cross(&vector2), Tuple::Vector(-1.0, 2.0, -1.0));
    /// ```
    pub fn cross(&self, other: &Self) -> Tuple {
        Tuple::Vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    /// ```
    /// use raytracer_rust::tuple::Tuple;
    /// let vector = Tuple::Vector(1.0, 2.0, 3.0);
    /// assert_eq!(vector.to_arr(), [1.0, 2.0, 3.0, 0.0]);
    /// ```
    pub fn to_arr(self) -> [f64; 4] {
        [self.x, self.y, self.z, self.w]
    }

    /// ```
    /// use raytracer_rust::tuple::Tuple;
    /// let v = Tuple::Vector(1.0, -1.0, 0.0);
    /// let n = Tuple::Vector(0.0, 1.0, 0.0);
    /// assert_eq!(v.reflect(n), Tuple::Vector(1.0, 1.0, 0.0));
    /// ```
    pub fn reflect(&self, normal: Tuple) -> Tuple {
        *self - normal * 2.0 * self.dot(&normal)
    }
}

impl Add for Tuple {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self::Output {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self
    }
}

impl AddAssign for Tuple {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Tuple {
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self::Output {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self.w -= rhs.w;

        self
    }
}

impl SubAssign for Tuple {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Neg for Tuple {
    type Output = Self;
    fn neg(mut self) -> Self::Output {
        self.x *= -1.0;
        self.y *= -1.0;
        self.z *= -1.0;
        self.w *= -1.0;

        self
    }
}

impl<T> Mul<T> for Tuple
where
    T: Into<f64> + Copy,
{
    type Output = Self;
    fn mul(mut self, rhs: T) -> Self::Output {
        self.x *= rhs.into();
        self.y *= rhs.into();
        self.z *= rhs.into();
        self.w *= rhs.into();

        self
    }
}

impl<T> Div<T> for Tuple
where
    T: Into<f64> + Copy,
{
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        self * (1.0 / rhs.into())
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        for (a, b) in [(self.x, other.x), (self.y, other.y), (self.z, other.z)] {
            if (a - b).abs() > EPSILON {
                return false;
            }
        }
        if (self.w - other.w).abs() > EPSILON {
            return false;
        }
        true
    }
}

impl From<Vec<f64>> for Tuple {
    fn from(v: Vec<f64>) -> Self {
        Self {
            x: v[0],
            y: v[1],
            z: v[2],
            w: v[3],
        }
    }
}

impl From<[f64; 4]> for Tuple {
    fn from(v: [f64; 4]) -> Self {
        Self {
            x: v[0],
            y: v[1],
            z: v[2],
            w: v[3],
        }
    }
}

impl Display for Tuple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {} {}", self.x, self.y, self.z, self.w)
    }
}
