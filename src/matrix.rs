use std::{
    fmt::{Debug, Formatter},
    ops::{Div, Index, IndexMut, Mul},
};

#[derive(Clone, Debug)]
pub(crate) struct Matrix2D {
    pub inner: Vec<Vec<f64>>,
}

struct Slice<T>(T); // typically a row or column

// TODO: replace all(max) Vec::new() with Vec::with_capacity()
// can be done it many places as we already nrows,ncols
// ex: in new() ~~~let mut mat = Vec::new()~~~ => to Vec::with_capacity(nrows)
impl Matrix2D {
    pub(crate) fn new() -> Self {
        Self {
            inner: {
                let mut mat = Vec::new();
                for _j in 0..4 {
                    let mut m = Vec::new();
                    for _i in 0..4 {
                        m.push(0.0);
                    }
                    mat.push(m);
                }
                mat
            },
        }
    }

    pub(crate) fn size(&self) -> (i64, i64) {
        (4, 4)
    }

    pub(crate) fn as_transpose(&self) -> Self {
        let inner = {
            let mut mat = Vec::new();
            for j in 0..4 {
                let mut m = Vec::new();
                for i in 0..4 {
                    m.push(self.inner[i as usize][j as usize]);
                }
                mat.push(m);
            }
            mat
        };
        Self { inner }
    }

    #[allow(non_snake_case)]
    pub(crate) fn as_T(&self) -> Self {
        self.as_transpose()
    }

    pub(crate) fn to_transpose(&mut self) {
        let inner = {
            let mut mat = Vec::new();
            for j in 0..4 {
                let mut m = Vec::new();
                for i in 0..4 {
                    m.push(self.inner[i as usize][j as usize]);
                }
                mat.push(m);
            }
            mat
        };
        *self = Self { inner };
    }

    #[allow(non_snake_case)]
    pub(crate) fn to_T(&mut self) {
        self.to_transpose()
    }

    fn row(&self, row_no: usize) -> Vec<f64> {
        let mut v = vec![];
        for i in 0..4 {
            let k = self.inner[row_no][i as usize];
            v.push(k);
        }
        v
    }

    fn col(&self, col_no: usize) -> Vec<f64> {
        let mut v = vec![];
        for i in 0..4 {
            let k = self.inner[i as usize][col_no];
            v.push(k);
        }
        v
    }

    pub(crate) fn cross(&self, other: Self) -> Self {
        let mut cross_mat = Vec::new();
        for j in 0..4 {
            let mut m = Vec::new();
            for i in 0..4 {
                let comp = Slice(other.col(i as usize)) * Slice(self.row(j as usize));
                let mut sum = Default::default();
                for i in comp {
                    sum = sum + i;
                }
                m.push(sum);
            }
            cross_mat.push(m);
        }
        Self { inner: cross_mat }
    }

    pub(crate) fn det(&self) -> f64 {
        if 4 == 2 && 4 == 2 {
            return self[0][0] * self[1][1] - self[0][1] * self[1][0];
        }
        let mut det = Default::default();
        for i in 0..4 {
            let sign = self.sign_at(0, i as usize);
            let num = self[0][i as usize];
            let cofactor = self.cofactor_of(i as usize, 0);
            det = det + (sign * num * cofactor);
        }
        det
    }

    pub(crate) fn sign_at(&self, x: usize, y: usize) -> f64 {
        (-1i8).pow((x + y) as u32).into()
    }

    pub(crate) fn cofactor_of(&self, x: usize, y: usize) -> f64 {
        let mut a = Vec::new();
        for j in 0..4 {
            if j as usize == y {
                continue;
            }
            let mut b = Vec::new();
            for i in 0..4 {
                if i as usize == x {
                    continue;
                }
                b.push(self.inner[j as usize][i as usize]);
            }
            a.push(b);
        }
        Self { inner: a }.det()
    }

    pub(crate) fn adjoint(&self) -> Self {
        let mut inner = Vec::new();
        for j in 0..4 {
            let mut m = Vec::new();
            for i in 0..4 {
                let sign = self.sign_at(i as usize, j as usize);
                m.push(sign * self.cofactor_of(i as usize, j as usize));
            }
            inner.push(m);
        }
        let mut adjoint_mat = Self { inner };
        adjoint_mat.to_T();
        adjoint_mat
    }

    pub(crate) fn inverse(&self) -> Matrix2D {
        let det = self.det();
        if det == 0.0 {
            panic!("Cannot invert this shit")
        }
        self.adjoint() / self.det()
    }
}

impl Index<usize> for Matrix2D {
    type Output = Vec<f64>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

impl IndexMut<usize> for Matrix2D {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.inner[index]
    }
}

impl Div<f64> for Matrix2D {
    type Output = Matrix2D;
    fn div(mut self, rhs: f64) -> Self::Output {
        let mut mat = Vec::new();
        for j in self.inner.iter_mut() {
            let mut m = Vec::new();
            for i in j.iter_mut() {
                m.push(f64::from(*i) / f64::from(rhs));
            }
            mat.push(m)
        }
        Matrix2D { inner: mat }
    }
}

impl<T> From<Vec<Vec<T>>> for Matrix2D
where
    f64: From<T>,
    T: Copy,
{
    fn from(vec: Vec<Vec<T>>) -> Self {
        Self {
            inner: {
                let mut mat = Vec::new();
                for j in 0..4 {
                    let mut m = Vec::new();
                    for i in 0..4 {
                        m.push(vec[j][i].into());
                    }
                    mat.push(m);
                }
                mat
            },
        }
    }
}

// TODO: fix spacing problem
// HELP: instead of using `out.push_str("  ");` two times ,
// convert T.to_string() and then add required no.of spaces
// ex: 2.to_string() gives "2" => add 3 spaces   => "   2" // now they look uniform
// ex: 34.to_string() gives "34" => add 2 spaces => "  34" // now they look uniform
impl std::fmt::Display for Matrix2D {
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
