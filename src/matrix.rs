use crate::{consts::EPSILON, tuple::Tuple, RoundToNDecimalPlaces};
use std::{
    fmt::{Debug, Display, Formatter},
    ops::{Div, Index, IndexMut, Mul},
};

#[derive(Clone, Debug, Copy)]
pub struct Matrix<const N: usize> {
    inner: [[f64; N]; N],
}

struct Slice<T>(T); // typically a row or column

impl<const N: usize> Matrix<N> {
    #[inline]
    pub fn new(arr: [[f64; N]; N]) -> Self {
        Self { inner: arr }
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<[f64; N]> {
        self.inner.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> std::slice::IterMut<[f64; N]> {
        self.inner.iter_mut()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    #[inline]
    pub const fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn row(&self, row_no: usize) -> [f64; N] {
        *self.iter().nth(row_no).unwrap()
    }

    pub fn col(&self, col_no: usize) -> [f64; N] {
        let mut col = [0.0; N];
        self.iter()
            .zip(col.iter_mut())
            .for_each(|(v, u)| *u = v[col_no]);
        col
    }

    /// ```
    /// use raytracer_rust::matrix::Matrix;
    /// let m = Matrix::new([[1.0, 2.0, 3.0], [3.0, 2.0, 1.0], [3.0, 1.0, 2.0]]);
    /// assert_eq!(m.cross(m), Matrix::<3>::new([[16.0, 9.0, 11.0], [12.0, 11.0, 13.0], [12.0, 10.0, 14.0]]));
    /// let m1 = Matrix::<4>::new([
    ///     [1.0, 2.0, 3.0, 4.0],
    ///     [5.0, 6.0, 7.0, 8.0],
    ///     [9.0, 8.0, 7.0, 6.0],
    ///     [5.0, 4.0, 3.0, 2.0]]);
    /// let m2 = Matrix::<4>::new([
    ///     [-2.0, 1.0, 2.0, 3.0],
    ///     [3.0, 2.0, 1.0, -1.0],
    ///     [4.0, 3.0, 6.0, 5.0],
    ///     [1.0, 2.0, 7.0, 8.0]]);
    /// assert_eq!(m1.cross(m2), Matrix::<4>::new([
    ///     [20.0, 22.0, 50.0, 48.0],
    ///     [44.0, 54.0, 114.0, 108.0],
    ///     [40.0, 58.0, 110.0, 102.0],
    ///     [16.0, 26.0, 46.0, 42.0]]));
    /// ```
    pub fn cross(&self, other: Self) -> Self {
        let mut cross_mat = Self::default();
        for j in 0..self.len() {
            for i in 0..self.len() {
                let comp = Slice(other.col(i as usize)) * Slice(self.row(j as usize));
                let mut sum = f64::default();
                for i in comp {
                    sum += i;
                }
                cross_mat[j][i] = sum;
            }
        }
        cross_mat
    }
}
impl Matrix<4> {
    #[allow(non_snake_case)]
    pub fn Identity() -> Matrix<4> {
        Matrix {
            inner: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    #[rustfmt::skip]
    #[allow(non_snake_case)]
    pub fn Translation<T:Into<f64>>(x: T, y: T, z: T) -> Matrix<4>
    {
        Matrix::new([
            [1.0, 0.0, 0.0, x.into()],
            [0.0, 1.0, 0.0, y.into()],
            [0.0, 0.0, 1.0, z.into()],
            [0.0, 0.0, 0.0,    1.0  ],
        ])
    }

    #[rustfmt::skip]
    #[allow(non_snake_case)]
    pub fn Scaling<T:Into<f64>>(x: T, y: T, z: T) -> Matrix<4>
    {
        Matrix::new([
            [x.into(), 0.0,      0.0,      0.0],
            [0.0,      y.into(), 0.0,      0.0],
            [0.0,      0.0,      z.into(), 0.0],
            [0.0,      0.0,      0.0,      1.0],
        ])
    }

    /// ```
    /// use raytracer_rust::matrix::Matrix;
    /// use raytracer_rust::tuple::Tuple;
    /// let shearing = Matrix::<4>::Shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    /// let point = Tuple::Point(2.0, 3.0, 4.0);
    /// let expected = Tuple::Point(5.0, 3.0, 4.0);
    /// assert_eq!(shearing * point, expected);
    /// ```
    #[rustfmt::skip]
    #[allow(non_snake_case)]
    pub fn Shearing<T: Into<f64>>(xy: T, xz: T, yx: T, yz: T, zx: T, zy: T) -> Matrix<4>
    {
        Matrix::new([
            [1.0,       xy.into(),  xz.into(), 0.0],
            [yx.into(), 1.0,        yz.into(), 0.0],
            [zx.into(), zy.into(),  1.0,       0.0],
            [0.0,       0.0,        0.0,       1.0],
        ])
    }

    pub fn transpose(&self) -> Self {
        let mut mat = Self::default();
        for j in 0..self.len() {
            for i in 0..self.len() {
                mat[j][i] = self.inner[i][j];
            }
        }
        mat
    }

    /// ```
    /// use raytracer_rust::matrix::Matrix;
    /// let m = Matrix::<4>::new([
    ///    [-2.0, -8.0, 3.0, 5.0],
    ///    [-3.0, 1.0, 7.0, 3.0],
    ///    [1.0, 2.0, -9.0, 6.0],
    ///    [-6.0, 7.0, 7.0, -9.0]]);
    /// assert_eq!(m.determinant(), -4071.0);
    /// ```
    pub fn determinant(&self) -> f64 {
        let mut det = f64::default();
        for i in 0..self.inner.len() {
            let sign = self.sign_at(0, i);
            let num = self[0][i];
            let cofactor = self.cofactor_of(i, 0);
            det += sign * num * cofactor;
        }
        det
    }

    pub fn sign_at(&self, x: usize, y: usize) -> f64 {
        // TODO: use bitwise operations or even-odd distinguation
        (-1i8).pow((x + y) as u32).into()
    }

    pub fn cofactor_of(&self, x: usize, y: usize) -> f64 {
        let mut cof_mat = Matrix::<3>::default();
        let mut after_x = 0;
        let mut after_y = 0;
        for j in 0..self.len() {
            if j as usize == y {
                after_y += 1;
                continue;
            }
            for i in 0..self.len() {
                if i as usize == x {
                    after_x += 1;
                    continue;
                }
                cof_mat[j - after_y][i - after_x] = self.inner[j as usize][i as usize];
            }
            after_x = 0;
        }
        cof_mat.determinant()
    }

    pub fn adjoint(&self) -> Self {
        let mut adjoint_mat = Self::default();
        for j in 0..self.len() {
            for i in 0..self.len() {
                let sign = self.sign_at(i as usize, j as usize);
                adjoint_mat[j][i] = sign * self.cofactor_of(i as usize, j as usize);
            }
        }
        adjoint_mat.transpose()
    }

    /// ```
    /// use raytracer_rust::matrix::Matrix;
    /// let m = Matrix::<4>::new([
    ///   [6.0, 4.0, 4.0, 4.0],
    ///   [5.0, 5.0, 7.0, 6.0],
    ///   [4.0, -9.0, 3.0, -7.0],
    ///   [9.0, 1.0, 7.0, -6.0]]);
    /// assert_eq!(m.determinant(), -2120.0);
    /// assert_eq!(m.is_invertible(), true);
    /// let m = Matrix::<4>::new([
    ///   [-4.0, 2.0, -2.0, -3.0],
    ///   [9.0, 6.0, 2.0, 6.0],
    ///   [0.0, -5.0, 1.0, -5.0],
    ///   [0.0, 0.0, 0.0, 0.0]]);
    /// assert_eq!(m.determinant(), 0.0);
    /// assert_eq!(m.is_invertible(), false);
    /// ```
    pub fn is_invertible(&self) -> bool {
        self.determinant() != 0.0
    }

    /// ```
    /// use raytracer_rust::matrix::Matrix;
    /// let m = Matrix::<4>::new([
    ///  [-5.0, 2.0, 6.0, -8.0],
    ///  [1.0, -5.0, 1.0, 8.0],
    ///  [7.0, 7.0, -6.0, -7.0],
    ///  [1.0, -3.0, 7.0, 4.0]]);
    /// assert_eq!(m.determinant(), 532.0);
    /// assert!(m.is_invertible());
    /// let inv = m.inverse();
    /// let res = Matrix::new([
    ///  [ 0.21805,  0.45113 , 0.24060 , -0.04511 ],
    ///  [ -0.80827,  -1.45677 , -0.44361 , 0.52068 ],
    ///  [ -0.07895,  -0.22368 , -0.05263 , 0.19737 ],
    ///  [ -0.52256,  -0.81391 , -0.30075 , 0.30639 ]]);
    /// assert_eq!(inv, res);
    /// ```
    pub fn inverse(&self) -> Matrix<4> {
        if self.is_invertible() {
            self.adjoint() / self.determinant()
        } else {
            panic!("Cannot invert a Matrix with determinant = 0\n")
        }
    }

    /// ```
    /// use raytracer_rust::tuple::Tuple;
    /// use raytracer_rust::matrix::Matrix;
    /// use std::f64::consts::FRAC_PI_4;
    /// use std::f64::consts::FRAC_PI_2;
    /// use std::f64::consts::SQRT_2;
    /// let p = Tuple::Point(0.0, 1.0, 0.0);
    /// let half_quarter = Matrix::rotation_X(FRAC_PI_4);
    /// let full_quarter = Matrix::rotation_X(FRAC_PI_2);
    /// assert_eq!(half_quarter * p, Tuple::Point(0.0, SQRT_2 / 2.0, SQRT_2 / 2.0));
    /// assert_eq!(full_quarter * p, Tuple::Point(0.0, 0.0, 1.0));
    /// ```
    #[allow(non_snake_case)]
    #[rustfmt::skip]
    pub fn rotation_X(x: f64) -> Matrix<4> {
        Matrix::from([
            [1.0,   0.0,     0.0,    0.0],
            [0.0, x.cos(), -x.sin(), 0.0],
            [0.0, x.sin(), x.cos(),  0.0],
            [0.0,   0.0,     0.0,    1.0],
        ])
    }

    /// ```
    /// use raytracer_rust::tuple::Tuple;
    /// use raytracer_rust::matrix::Matrix;
    /// use std::f64::consts::FRAC_PI_4;
    /// use std::f64::consts::FRAC_PI_2;
    /// use std::f64::consts::SQRT_2;
    /// let p = Tuple::Point(0.0, 0.0, 1.0);
    /// let half_quarter = Matrix::rotation_Y(FRAC_PI_4);
    /// let full_quarter = Matrix::rotation_Y(FRAC_PI_2);
    /// assert_eq!(half_quarter * p, Tuple::Point(SQRT_2 / 2.0, 0.0, SQRT_2 / 2.0));
    /// assert_eq!(full_quarter * p, Tuple::Point(1.0, 0.0, 0.0));
    /// ```
    #[allow(non_snake_case)]
    #[rustfmt::skip]
    pub fn rotation_Y(x: f64) -> Matrix<4> {
        Matrix::from([
            [x.cos(),  0.0, x.sin(), 0.0],
            [0.0,      1.0, 0.0,     0.0],
            [-x.sin(), 0.0, x.cos(), 0.0],
            [0.0,      0.0, 0.0,     1.0],
        ])
    }

    /// ```
    /// use raytracer_rust::tuple::Tuple;
    /// use raytracer_rust::matrix::Matrix;
    /// use std::f64::consts::FRAC_PI_4;
    /// use std::f64::consts::FRAC_PI_2;
    /// use std::f64::consts::SQRT_2;
    /// let p = Tuple::Point(0.0, 1.0, 0.0);
    /// let half_quarter = Matrix::rotation_Z(FRAC_PI_4);
    /// let full_quarter = Matrix::rotation_Z(FRAC_PI_2);
    /// assert_eq!(half_quarter * p, Tuple::Point(-SQRT_2 / 2.0, SQRT_2 / 2.0, 0.0));
    /// assert_eq!(full_quarter * p, Tuple::Point(-1.0, 0.0, 0.0));
    /// ```
    #[allow(non_snake_case)]
    #[rustfmt::skip]
    pub fn rotation_Z(x: f64) -> Matrix<4> {
        Matrix::from([
            [x.cos(), -x.sin(), 0.0, 0.0],
            [x.sin(),  x.cos(), 0.0, 0.0],
            [  0.0,     0.0,    1.0, 0.0],
            [  0.0,     0.0,    0.0, 1.0],
        ])
    }

    /// ```
    /// use raytracer_rust::tuple::Tuple;
    /// use raytracer_rust::matrix::Matrix;
    /// let from = Tuple::Point(0.0, 0.0, 8.0);
    /// let to = Tuple::Point(0.0, 0.0, 0.0);
    /// let up = Tuple::Vector(0.0, 1.0, 0.0);
    /// let t = Matrix::view_transform(from, to, up);
    /// assert_eq!(t, Matrix::Translation(0.0, 0.0, -8.0));
    ///
    /// let from = Tuple::Point(1.0, 3.0, 2.0);
    /// let to = Tuple::Point(4.0, -2.0, 8.0);
    /// let up = Tuple::Vector(1.0, 1.0, 0.0);
    /// let t = Matrix::view_transform(from, to, up);
    /// assert_eq!(t, Matrix::from([
    ///    [ -0.50709,  0.50709,  0.67612, -2.36643],
    ///    [  0.76772,  0.60609,  0.12122, -2.82843],
    ///    [ -0.35857,  0.59761, -0.71714,  0.00000],
    ///    [  0.00000,  0.00000,  0.00000,  1.00000],
    ///     ]));
    ///
    /// ```
    pub fn view_transform(from: Tuple, to: Tuple, up: Tuple) -> Matrix<4> {
        let forward = (to - from).normalize();
        let left = forward.cross(&up.normalize());
        let true_up = left.cross(&forward);
        #[rustfmt::skip]
        let orientation = Matrix::from([
            [left.x,     left.y,     left.z,     0.0],
            [true_up.x,  true_up.y,  true_up.z,  0.0],
            [-forward.x, -forward.y, -forward.z, 0.0],
            [0.0,        0.0,        0.0,        1.0],
        ]);
        orientation * Matrix::Translation(-from.x, -from.y, -from.z)
    }
}

impl Matrix<3> {
    /// ```
    /// use raytracer_rust::matrix::Matrix;
    /// let m = Matrix::<3>::new([
    ///    [1.0, 2.0, 6.0],
    ///    [-5.0, 8.0, -4.0],
    ///    [2.0, 6.0, 4.0]]);
    /// assert_eq!(m.determinant(), -196.0);
    /// ```
    #[rustfmt::skip]
    pub fn determinant(&self) -> f64 {
              self[0][0] * ((self[1][1] * self[2][2]) - (self[1][2] * self[2][1]))
            - self[0][1] * ((self[1][0] * self[2][2]) - (self[1][2] * self[2][0]))
            + self[0][2] * ((self[1][0] * self[2][1]) - (self[1][1] * self[2][0]))
    }
}

impl Matrix<2> {
    /// ```
    /// use raytracer_rust::matrix::Matrix;
    /// let m = Matrix::<2>::new([
    ///    [1.0, 5.0],
    ///    [-3.0, 2.0]]);
    /// assert_eq!(m.determinant(), 17.0);
    /// ```
    pub fn determinant(&self) -> f64 {
        (self[0][0] * self[1][1]) - (self[0][1] * self[1][0])
    }
}

impl<const N: usize> Default for Matrix<N> {
    fn default() -> Self {
        Matrix::new([[0.0; N]; N])
    }
}

impl<const N: usize> PartialEq for Matrix<N> {
    fn eq(&self, rhs: &Self) -> bool {
        for i in 0..N {
            for j in 0..N {
                if (self[i][j] - rhs[i][j]).abs() > EPSILON {
                    return false;
                }
            }
        }
        true
    }
}

impl<const N: usize> Index<usize> for Matrix<N> {
    type Output = [f64; N];
    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

impl<const N: usize> IndexMut<usize> for Matrix<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.inner[index]
    }
}

impl<const N: usize> Mul<Matrix<N>> for Matrix<N> {
    type Output = Self;
    fn mul(self, rhs: Matrix<N>) -> Self::Output {
        self.cross(rhs)
    }
}

impl<const N: usize> IntoIterator for Matrix<N> {
    type Item = [f64; N];
    type IntoIter = std::array::IntoIter<Self::Item, N>;
    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

// FIXME: code duplication
// From<Vec<Vec<T>>> for Matrix<N>
// From<[[T; N]; N]> for Matrix<N>
// HOWTO: Use macros to define the following impls.
impl<T, const N: usize> From<Vec<Vec<T>>> for Matrix<N>
where
    T: Into<f64> + Copy,
{
    fn from(v: Vec<Vec<T>>) -> Self {
        let mut mat = Self::default();
        for j in 0..v.len() {
            for i in 0..v[0].len() {
                mat[j][i] = v[j][i].into();
            }
        }
        mat
    }
}

impl<T, const N: usize> From<[[T; N]; N]> for Matrix<N>
where
    T: Into<f64> + Copy,
{
    fn from(v: [[T; N]; N]) -> Self {
        let mut mat = Self::default();
        for j in 0..v.len() {
            for i in 0..v[0].len() {
                mat[j][i] = v[j][i].into();
            }
        }
        mat
    }
}

// FIXME: check: fix spacing problem
impl<const N: usize> Display for Matrix<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        let max_chars = 6;
        for j in self.inner.iter() {
            for i in j.iter() {
                let i_as_str = i.round_to_n_decimal_places(2).to_string();
                out.push_str(&i_as_str);
                out.push_str(&" ".repeat(max_chars - i_as_str.len()));
            }
            out.push('\n');
        }
        write!(f, "{}", out)
    }
}

impl<const N: usize> Div<f64> for Matrix<N> {
    type Output = Matrix<N>;
    fn div(self, rhs: f64) -> Self::Output {
        let mut mat = Self::default();
        for (j, row) in self.into_iter().enumerate() {
            for (i, val) in row.into_iter().enumerate() {
                mat[j][i] = val / rhs;
            }
        }
        mat
    }
}

impl<T> Mul<T> for Slice<Vec<T>>
where
    T: std::ops::Mul<Output = T> + Copy,
{
    type Output = Vec<T>;
    fn mul(mut self, rhs: T) -> Self::Output {
        for i in self.0.iter_mut() {
            *i = *i * rhs;
        }
        self.0
    }
}

// FIXME: code duplication
// Mul<Slice<Vec<T>>> for Slice<Vec<T>>
// Mul<Slice<[T; N]>> for Slice<[T; N]>
// Mul<Slice<[T; N]>> for Slice<Vec<T>>
// HOWTO: Use macros to define the following impls.
impl<T> Mul<Slice<Vec<T>>> for Slice<Vec<T>>
where
    T: std::ops::Mul<Output = T> + Copy,
{
    type Output = Vec<T>;
    fn mul(mut self, rhs: Slice<Vec<T>>) -> Self::Output {
        for (i, j) in self.0.iter_mut().zip(rhs.0.into_iter()) {
            *i = *i * j;
        }
        self.0
    }
}

impl<T, const N: usize> Mul<Slice<[T; N]>> for Slice<[T; N]>
where
    T: Copy + std::ops::Mul<Output = T>,
{
    type Output = [T; N];
    fn mul(mut self, rhs: Slice<[T; N]>) -> Self::Output {
        for (i, j) in self.0.iter_mut().zip(rhs.0.into_iter()) {
            *i = *i * j;
        }
        self.0
    }
}

impl<T, const N: usize> Mul<Slice<[T; N]>> for Slice<Vec<T>>
where
    T: std::ops::Mul<Output = T> + Copy,
{
    type Output = Vec<T>;
    fn mul(mut self, rhs: Slice<[T; N]>) -> Self::Output {
        assert_eq!(self.0.len(), N);
        for (i, j) in self.0.iter_mut().zip(rhs.0.into_iter()) {
            *i = *i * j;
        }
        self.0
    }
}

/// ```
/// use raytracer_rust::matrix::Matrix;
/// use raytracer_rust::tuple::Tuple;
/// let t = Tuple::new(1.0, 2.0, 3.0, 1.0);
/// let m = Matrix::new([
///     [1.0, 2.0, 3.0, 4.0],
///     [2.0, 4.0, 4.0, 2.0],
///     [8.0, 6.0, 4.0, 1.0],
///     [0.0, 0.0, 0.0, 1.0]]);
/// assert_eq!(m * t, Tuple::new(18.0, 24.0, 33.0, 1.0));
///
/// let transform = Matrix::Translation(5.0, -3.0, 2.0);
/// let p = Tuple::Point(-3.0, 4.0, 5.0);
/// assert_eq!(transform * p, Tuple::Point(2.0, 1.0, 7.0));
/// assert_eq!(transform.inverse() * p, Tuple::Point(-8.0, 7.0, 3.0));
///
/// let v = Tuple::Vector(-3.0, 4.0, 5.0);
/// assert_eq!(transform * v, Tuple::Vector(-3.0, 4.0, 5.0));
///
/// let transform = Matrix::Scaling(2.0, 3.0, 4.0);
/// let p = Tuple::Point(-4.0, 6.0, 8.0);
/// let v = Tuple::Vector(-4.0, 6.0, 8.0);
/// assert_eq!(transform * p, Tuple::Point(-8.0, 18.0, 32.0));
/// assert_eq!(transform * v, Tuple::Vector(-8.0, 18.0, 32.0));
/// assert_eq!(transform.inverse() * p, Tuple::Point(-2.0, 2.0, 2.0));
/// ```
impl Mul<Tuple> for Matrix<4> {
    type Output = Tuple;
    fn mul(self, rhs: Tuple) -> Self::Output {
        let mut v = [0.0; 4];
        for (i, row) in self.inner.into_iter().enumerate() {
            let k = Slice(row) * Slice(rhs.to_arr());
            let comp = k.into_iter().sum();
            v[i] = comp;
        }

        Tuple::from(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::consts::PI_BY_2;

    #[test]
    #[allow(non_snake_case)]
    fn matrix_cross_Identity_works() {
        let m = Matrix::new([
            [0.0, 1.0, 2.0, 4.0],
            [1.0, 2.0, 4.0, 8.0],
            [2.0, 4.0, 8.0, 16.0],
            [4.0, 8.0, 16.0, 32.0],
        ]);
        let I = Matrix::Identity();
        let r = m * I;
        assert_eq!(r, m);
    }

    #[test]
    fn product_test_works() {
        let a = Matrix::new([
            [3.0, -9.0, 7.0, 3.0],
            [3.0, -8.0, 2.0, -9.0],
            [-4.0, 4.0, 4.0, 1.0],
            [-6.0, 5.0, -1.0, 1.0],
        ]);
        let b = Matrix::new([
            [8.0, 2.0, 2.0, 2.0],
            [3.0, -1.0, 7.0, 0.0],
            [7.0, 0.0, 5.0, 4.0],
            [6.0, -2.0, 0.0, 5.0],
        ]);
        let c = a * b;
        assert_eq!(c * b.inverse(), a)
    }

    #[test]
    fn chaining_works() {
        let p = Tuple::Point(1.0, 0.0, 1.0);
        let a = Matrix::rotation_X(PI_BY_2);
        let b = Matrix::Scaling(5.0, 5.0, 5.0);
        let c = Matrix::Translation(10.0, 5.0, 7.0);
        assert_eq!(c * b * a * p, Tuple::Point(15.0, 0.0, 7.0));
    }
}
