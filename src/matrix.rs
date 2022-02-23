use crate::tuple::Tuple;
use std::{
    fmt::{Debug, Formatter},
    ops::{Div, Index, IndexMut, Mul},
    slice::{Iter, IterMut},
};

/// TODO: Improve Matrix struct by changing the type of inner
/// can use 1D vec ` Vec<f64> ` instead of 2D vec `Vec<Vec<f64>>`
/// ? also why to use vecs ?
/// try to use arrays instead, tried and failed
/// ```
/// let len = 4;
/// let arr = [0; len]; //Error: attempt to use a non-constant value in a constant non-constant value
/// ```
/// * try using unsafe blocks and pointers
#[derive(Clone, Debug)]
pub(crate) struct Matrix {
    pub(crate) inner: Vec<Vec<f64>>,
}
pub(crate) struct Translation;
pub(crate) struct Scaling;
pub(crate) enum Rotation {
    X(f64),
    Y(f64),
    Z(f64),
}
pub(crate) struct Shearing;
struct Slice<T>(T); // typically a row or column

impl Matrix {
    pub(crate) fn new(size: usize) -> Self {
        Self {
            inner: vec![vec![0.0; size]; size],
        }
    }

    #[allow(non_snake_case)]
    pub(crate) fn Identity() -> Matrix {
        Matrix {
            inner: vec![
                vec![1.0, 0.0, 0.0, 0.0],
                vec![0.0, 1.0, 0.0, 0.0],
                vec![0.0, 0.0, 1.0, 0.0],
                vec![0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    #[inline]
    pub(crate) fn len(&self) -> usize {
        self.inner.len()
    }

    #[inline]
    pub(crate) fn size(&self) -> (usize, usize) {
        (self.inner[0].len(), self.inner.len())
    }

    pub(crate) fn transpose(&self) -> Self {
        let mut mat = Self::new(self.len());
        for j in 0..self.len() {
            for i in 0..self.len() {
                mat[j][i] = self.inner[i as usize][j as usize];
            }
        }
        mat
    }

    #[allow(non_snake_case)]
    pub(crate) fn T(&self) -> Self {
        self.transpose()
    }

    pub(crate) fn to_transpose(&mut self) {
        let mut mat = Self::new(self.len());
        for j in 0..self.len() {
            for i in 0..self.len() {
                mat[j][i] = self.inner[i as usize][j as usize];
            }
        }
        *self = mat;
    }

    #[allow(non_snake_case)]
    pub(crate) fn to_T(&mut self) {
        self.to_transpose()
    }

    pub(crate) fn row(&self, row_no: usize) -> Vec<f64> {
        // let mut row = Vec::with_capacity(self.len());
        // for i in 0..self.len() {
        //     row.push(self[row_no][i as usize]);
        // }
        // clippy recommended
        self.iter().nth(row_no).unwrap().clone()
        // row
    }

    pub(crate) fn col(&self, col_no: usize) -> Vec<f64> {
        // let mut col = Vec::with_capacity(self.len());
        // for i in 0..self.len() {
        //     col.push(self[i as usize][col_no]);
        // }
        // clippy recommended
        self.iter().map(|v| (v[col_no])).collect()
        // col
    }

    pub(crate) fn cross(&self, other: Self) -> Self {
        let mut cross_mat = Self::new(self.len());
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

    pub(crate) fn det(&self) -> f64 {
        if self.inner.len() == 2 {
            return self[0][0] * self[1][1] - self[0][1] * self[1][0];
        }
        let mut det = f64::default();
        for i in 0..(self.inner.len()) {
            let sign = self.sign_at(0, i as usize);
            let num = self[0][i as usize];
            let cofactor = self.cofactor_of(i as usize, 0);
            det += sign * num * cofactor;
        }
        det
    }

    pub(crate) fn sign_at(&self, x: usize, y: usize) -> f64 {
        (-1i8).pow((x + y) as u32).into()
    }

    pub(crate) fn cofactor_of(&self, x: usize, y: usize) -> f64 {
        let size = self.inner.len() - 1;
        let mut cof_mat = Matrix::new(size);
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
        cof_mat.det()
    }

    pub(crate) fn adjoint(&self) -> Self {
        let mut adjoint_mat = Self::new(self.len());
        for j in 0..self.len() {
            for i in 0..self.len() {
                let sign = self.sign_at(i as usize, j as usize);
                adjoint_mat[j][i] = sign * self.cofactor_of(i as usize, j as usize);
            }
        }
        adjoint_mat.to_T();
        adjoint_mat
    }

    pub(crate) fn inverse(&self) -> Matrix {
        let det = self.det();
        if det == 0.0 {
            panic!("Cannot invert a Matrix with determinant = 0")
        }
        self.adjoint() / self.det()
    }

    pub(crate) fn iter(&self) -> Iter<Vec<f64>> {
        self.inner.iter()
    }

    pub(crate) fn iter_mut(&mut self) -> IterMut<Vec<f64>> {
        self.inner.iter_mut()
    }
}

impl Index<usize> for Matrix {
    type Output = Vec<f64>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.inner[index]
    }
}

impl Div<f64> for Matrix {
    type Output = Matrix;
    fn div(self, rhs: f64) -> Self::Output {
        let mut mat = Self::new(4);
        for (j, row) in self.iter().enumerate() {
            for (i, val) in row.iter().enumerate() {
                mat[j][i] = (*val) / rhs;
            }
        }
        mat
    }
}

impl Mul<Matrix> for Matrix {
    type Output = Self;
    fn mul(self, rhs: Matrix) -> Self::Output {
        self.cross(rhs)
    }
}

impl IntoIterator for Matrix {
    type Item = Vec<f64>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<T> From<Vec<Vec<T>>> for Matrix
where
    f64: From<T>,
    T: Copy,
{
    fn from(v: Vec<Vec<T>>) -> Self {
        let mut mat = Self::new(v.len());
        for j in 0..v.len() {
            for i in 0..v[0].len() {
                mat[j][i] = v[j][i].into();
            }
        }
        mat
    }
}

// check: fix spacing problem
impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        let max_chars = 5;
        for j in self.inner.iter() {
            for i in j.iter() {
                let i_as_str = i.to_string();
                out.push_str(&i_as_str);
                out.push_str(&" ".repeat(max_chars - i_as_str.len()));
            }
            out.push('\n')
        }
        write!(f, "{}", out)
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

impl<T> Mul<Slice<Vec<T>>> for Slice<Vec<T>>
where
    T: std::ops::Mul<Output = T> + Copy,
{
    type Output = Vec<T>;
    fn mul(mut self, rhs: Slice<Vec<T>>) -> Self::Output {
        for (i, j) in self.0.iter_mut().zip(rhs.0.iter()) {
            *i = *i * *j;
        }
        self.0
    }
}

impl Mul<Tuple> for Matrix {
    type Output = Tuple;
    fn mul(self, rhs: Tuple) -> Self::Output {
        let mut v = Vec::new();
        for i in self.inner.into_iter() {
            let k = Slice(i) * Slice(rhs.to_vec());
            let mut comp = 0.0;
            for j in k.iter() {
                comp += *j;
            }
            v.push(comp)
        }

        Tuple::from(v)
    }
}

impl Mul<Tuple> for &Matrix {
    type Output = Tuple;
    fn mul(self, rhs: Tuple) -> Self::Output {
        let mut v = Vec::new();
        for i in self.inner.iter() {
            let k = Slice(i.clone()) * Slice(rhs.to_vec());
            let mut comp = 0.0;
            for j in k.iter() {
                comp += *j;
            }
            v.push(comp)
        }

        Tuple::from(v)
    }
}

impl<T, const N: usize> Mul<Slice<Vec<T>>> for Slice<[T; N]>
where
    f64: From<T>,
    T: Copy + std::ops::Mul<Output = T>,
{
    type Output = [T; N];
    fn mul(mut self, rhs: Slice<Vec<T>>) -> Self::Output {
        for (i, j) in self.0.iter_mut().zip(rhs.0.iter()) {
            *i = *i * *j;
        }
        self.0
    }
}

impl Translation {
    pub(crate) fn new<T>(x: T, y: T, z: T) -> Matrix
    where
        f64: From<T>,
    {
        Matrix::from(vec![
            vec![1.0, 0.0, 0.0, f64::from(x)],
            vec![0.0, 1.0, 0.0, f64::from(y)],
            vec![0.0, 0.0, 1.0, f64::from(z)],
            vec![0.0, 0.0, 0.0, 1.0],
        ])
    }
}

impl Scaling {
    pub(crate) fn new<T>(x: T, y: T, z: T) -> Matrix
    where
        f64: From<T>,
    {
        Matrix::from(vec![
            vec![f64::from(x), 0.0, 0.0, 0.0],
            vec![0.0, f64::from(y), 0.0, 0.0],
            vec![0.0, 0.0, f64::from(z), 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ])
    }
}

impl Rotation {
    #[allow(non_snake_case)]
    pub(crate) fn newX(x: f64) -> Matrix {
        Matrix::from(vec![
            vec![1.0, 0.0, 0.0, 0.0],
            vec![0.0, x.cos(), -x.sin(), 0.0],
            vec![0.0, x.sin(), x.cos(), 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ])
    }

    #[allow(non_snake_case)]
    pub(crate) fn newY(x: f64) -> Matrix {
        Matrix::from(vec![
            vec![x.cos(), 0.0, x.sin(), 0.0],
            vec![0.0, 0.0, 0.0, 0.0],
            vec![-x.sin(), 0.0, x.cos(), 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ])
    }

    #[allow(non_snake_case)]
    pub(crate) fn newZ(x: f64) -> Matrix {
        Matrix::from(vec![
            vec![x.cos(), -x.sin(), 0.0, 0.0],
            vec![x.sin(), x.cos(), 0.0, 0.0],
            vec![0.0, 0.0, 1.0, 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ])
    }
}

impl Shearing {
    fn new(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix {
        Matrix::from(vec![
            vec![1.0, xy, xz, 0.0],
            vec![yx, 1.0, yz, 0.0],
            vec![zx, zy, 1.0, 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ])
    }
}
