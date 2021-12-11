#![allow(dead_code)]
use std::{
    fmt::{Debug, Display, Formatter},
    ops::{Div, Index, IndexMut, Mul},
};

#[derive(Clone, Debug)]
pub(crate) struct Matrix2D<T> {
    nrows: i64,
    ncols: i64,
    pub inner: Vec<Vec<T>>,
}

// TODO: replace all(max) Vec::new() with Vec::with_capacity()
// can be done it many places as we already nrows,ncols
// ex: in new() ~~~let mut mat = Vec::new()~~~ => to Vec::with_capacity(nrows)
impl<T> Matrix2D<T>
where
    T: Default
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>
        + Copy
        + Debug
        + Display
        + std::cmp::PartialEq
        + From<i8>,
    f64: From<T>,
{
    pub(crate) fn new(nrows: i64, ncols: i64) -> Self {
        Self {
            nrows,
            ncols,
            inner: {
                let mut mat = Vec::new();
                for _j in 0..(nrows) {
                    let mut m = Vec::new();
                    for _i in 0..(ncols) {
                        m.push(Default::default())
                    }
                    mat.push(m);
                }
                mat
            },
        }
    }

    pub(crate) fn as_transpose(&self) -> Self {
        let nrows = self.nrows;
        let ncols = self.ncols;
        let inner = {
            let mut mat = Vec::new();
            for j in 0..nrows {
                let mut m = Vec::new();
                for i in 0..ncols {
                    m.push(self.inner[i as usize][j as usize]);
                }
                mat.push(m);
            }
            mat
        };
        Self {
            nrows,
            ncols,
            inner,
        }
    }

    #[allow(non_snake_case)]
    pub(crate) fn as_T(&self) -> Self {
        self.as_transpose()
    }

    pub(crate) fn to_transpose(&mut self) {
        let nrows = self.nrows;
        let ncols = self.ncols;
        let inner = {
            let mut mat = Vec::new();
            for j in 0..nrows {
                let mut m = Vec::new();
                for i in 0..ncols {
                    m.push(self.inner[i as usize][j as usize]);
                }
                mat.push(m);
            }
            mat
        };
        *self = Self {
            nrows,
            ncols,
            inner,
        };
    }

    #[allow(non_snake_case)]
    pub(crate) fn to_T(&mut self) {
        self.to_transpose()
    }

    fn row(&self, row_no: usize) -> Vec<T> {
        let mut v = vec![];
        for i in 0..self.ncols {
            let k = self.inner[row_no][i as usize];
            v.push(k);
        }
        v
    }

    fn col(&self, col_no: usize) -> Vec<T> {
        let mut v = vec![];
        for i in 0..self.nrows {
            let k = self.inner[i as usize][col_no];
            v.push(k);
        }
        v
    }

    pub(crate) fn cross(&self, other: Self) -> Self {
        let mut cross_mat = Vec::new();
        for j in 0..self.ncols {
            let mut m = Vec::new();
            for i in 0..self.nrows {
                let comp = Slice(other.col(i as usize)) * Slice(self.row(j as usize));
                let mut sum = Default::default();
                for i in comp {
                    sum = sum + i;
                }
                m.push(sum);
            }
            cross_mat.push(m);
        }
        Self {
            nrows: self.nrows,
            ncols: self.ncols,
            inner: cross_mat,
        }
    }

    pub(crate) fn det(&self) -> T
    where
        T: From<i8>,
        f64: From<T>,
    {
        if self.nrows == 2 && self.ncols == 2 {
            return self[0][0] * self[1][1] - self[0][1] * self[1][0];
        }
        let mut det = Default::default();
        for i in 0..self.ncols {
            let sign = self.sign_at(0, i as usize);
            let num = self[0][i as usize];
            let cofactor = self.cofactor_of(i as usize, 0);
            det = det + (sign * num * cofactor);
        }
        // println!("for mat:\n{}\n det:{}\n", self, det);
        det
    }

    pub(crate) fn sign_at(&self, x: usize, y: usize) -> T
    where
        T: From<i8>,
    {
        T::from(((-1) as i8).pow((x + y) as u32))
    }

    pub(crate) fn cofactor_of(&self, x: usize, y: usize) -> T
    where
        T: From<i8>,
        f64: From<T>,
    {
        let mut a = Vec::new();
        for j in 0..self.nrows {
            if j as usize == y {
                continue;
            }
            let mut b = Vec::new();
            for i in 0..self.ncols {
                if i as usize == x {
                    continue;
                }
                b.push(self.inner[j as usize][i as usize]);
            }
            a.push(b);
        }
        let ncols = self.ncols - 1;
        let nrows = self.nrows - 1;
        Self {
            nrows,
            ncols,
            inner: a,
        }
        .det()
    }

    pub(crate) fn adjoint(&self) -> Self {
        let mut inner = Vec::new();
        for j in 0..self.nrows {
            let mut m = Vec::new();
            for i in 0..self.ncols {
                let sign = self.sign_at(i as usize, j as usize);
                m.push(sign * self.cofactor_of(i as usize, j as usize));
            }
            inner.push(m);
        }
        let mut adjoint_mat = Self {
            nrows: self.nrows,
            ncols: self.ncols,
            inner,
        };
        adjoint_mat.to_T();
        adjoint_mat
    }

    pub(crate) fn inverse(&self) -> Matrix2D<f64> {
        let det = self.det();
        if det == T::from(0i8) {
            panic!("Cannot invert this shit")
        }
        self.adjoint() / self.det()
    }
}

impl<T> Index<usize> for Matrix2D<T> {
    type Output = Vec<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

impl<T> IndexMut<usize> for Matrix2D<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.inner[index]
    }
}

impl<T> Div<T> for Matrix2D<T>
where
    T: std::ops::Div<Output = T> + Copy,
    f64: From<T>,
{
    type Output = Matrix2D<f64>;
    fn div(mut self, rhs: T) -> Self::Output {
        let mut mat = Vec::new();
        for j in self.inner.iter_mut() {
            let mut m = Vec::new();
            for i in j.iter_mut() {
                m.push(f64::from(*i) / f64::from(rhs));
            }
            mat.push(m)
        }
        Matrix2D {
            nrows: self.nrows,
            ncols: self.ncols,
            inner: mat,
        }
    }
}

struct Slice<T>(T); // typically a row or column

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

// TODO: fix spacing problem
// HELP: instead of using `out.push_str("  ");` two times ,
// convert T.to_string() and then add required no.of spaces
// ex: 2.to_string() gives "2" => add 3 spaces   => "   2" // now they look uniform
// ex: 34.to_string() gives "34" => add 2 spaces => "  34" // now they look uniform
impl<T> std::fmt::Display for Matrix2D<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        for j in self.inner.iter() {
            for i in j.iter() {
                out.push_str("  ");
                out.push_str(&i.to_string());
                out.push_str("  ");
            }
            out.push_str("\n")
        }
        write!(f, "{}", out)
    }
}
