use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};

#[derive(Clone, Debug)]
pub(crate) struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: f64, // w = 1 for Point , w = 0 for Vector
}
pub(crate) struct Point;
pub(crate) struct Vector;
// pub(crate) struct Scaling;
impl Tuple {
    pub(crate) fn new<T: Into<f64>>(x: T, y: T, z: T, w: T) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            z: z.into(),
            w: w.into(),
        }
    }
    pub(crate) fn is_point(&self) -> bool {
        self.w == 1.0
    }
    pub(crate) fn is_vector(&self) -> bool {
        !self.is_point()
    }
    pub(crate) fn magnitude(&self) -> f64 {
        assert_eq!(self.w, 0.0); // magnitude only exists for vectors
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    pub(crate) fn into_normalized_vector(self) -> Self {
        let m = self.magnitude();

        Tuple {
            x: self.x / m,
            y: self.y / m,
            z: self.z / m,
            w: self.w / m,
        }
    }
    pub(crate) fn as_normalized_vector(&self) -> Self {
        let m = self.magnitude();

        Self {
            x: self.x / m,
            y: self.y / m,
            z: self.z / m,
            w: self.w / m,
        }
    }
    pub(crate) fn to_normalized_vector(&mut self) -> &Self {
        let m = self.magnitude();

        self.x /= m;
        self.y /= m;
        self.z /= m;
        self.w /= m;
        self
    }
    pub(crate) fn dot(&self, other: &Self) -> f64 {
        assert_eq!(self.w, 0.0); // magnitude only exists for vectors
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub(crate) fn cross(&self, other: &Self) -> Tuple {
        Vector::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
    pub(crate) fn to_vec(&self) -> Vec<f64> {
        vec![self.x, self.y, self.z, self.w]
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
        // self.w = ((self.w - rhs.w) as i8).abs() as i8;
        if self.w == 1.0 && rhs.w == 1.0 {
            self.w = 0.0;
        } /* else {
              self.w = self.w.max(rhs.w);
          } */
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
        self.x = self.x * rhs.into();
        self.y = self.y * rhs.into();
        self.z = self.z * rhs.into();
        self.w = self.w * rhs.into();
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
        const EPSILON: f64 = 0.00001;
        for (a, b) in [(self.x, other.x), (self.y, other.y), (self.z, other.z)] {
            if (a - b).abs() > EPSILON {
                return false;
            }
        }
        if self.w != other.w {
            return false;
        }
        true
    }
}

impl PartialEq<Tuple> for &Tuple {
    fn eq(&self, other: &Tuple) -> bool {
        const EPSILON: f64 = 0.00001;
        for (a, b) in [(self.x, other.x), (self.y, other.y), (self.z, other.z)] {
            if (a - b).abs() > EPSILON {
                return false;
            }
        }
        if self.w != other.w {
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

impl Point {
    pub(crate) fn new<T: Into<f64>>(x: T, y: T, z: T) -> Tuple {
        Tuple {
            x: x.into(),
            y: y.into(),
            z: z.into(),
            w: 1.0,
        }
    }
}

impl Vector {
    pub(crate) fn new<T: Into<f64>>(x: T, y: T, z: T) -> Tuple {
        Tuple {
            x: x.into(),
            y: y.into(),
            z: z.into(),
            w: 0.0,
        }
    }
}
