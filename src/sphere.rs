use std::ops::Index;

use super::Ray;
#[derive(Debug, Clone)]
pub(crate) struct Sphere {
    r: usize,
    center: (f64, f64),
}

#[derive(Debug)]
pub(crate) struct Intersections<'a> {
    count: u8,
    i1: Intersection<'a>,
    i2: Intersection<'a>,
}

#[derive(Debug)]
pub(crate) struct Intersection<'a> {
    pub(crate) t: f64,
    pub(crate) object: &'a Sphere,
}

impl Sphere {
    pub(crate) fn new(r: usize) -> Self {
        Self {
            r,
            center: (0.0, 0.0),
        }
    }

    pub(crate) fn intersect(&self, r: Ray) -> Intersections {
        // todo!()
        // origin: (Ox, Oy,Oz)
        // let k: (f64, f64, f64, f64) = r.origin.into();
        // let (ox, oy, oz, _ow) = r.origin.as_tuple();
        // direction: (dx,dy,dz)
        // let (dx, dy, dz, _dw) = r.direction.as_tuple();
        // any point on ray: ( Ox+(dx*t), Oy+(dy*t), Oz+(dz*t) )
        // if distance of this point from sphere center is r then
        // this point lies on the sphere
        // eqn: x^2 + y^2 + z^2 = r ^2
        // substituting and rearranging the terms gives
        // (dx^2+dy^2+dz^2)t^2 + 2(Oxdx+Oydy+Ozdz)t + (Ox^2+Oy^2+Oz^2-r^2) = 0
        let a = r.direction.dot(&r.direction); // dx.powi(2) + dy.powi(2) + dz.powi(2);
        let b = 2.0 * r.origin.dot(&r.direction); // 2.0 * (ox * dx + oy * dy + oz * dz);
        let c = r.origin.dot(&r.origin) - (self.r as f64).powi(2); // ox.powi(2) + oy.powi(2) + oz.powi(2) - (self.r as f64).powi(2);
        let disc = b * b - 4.0 * a * c;
        let d1 = (-b - disc.sqrt()) / 2.0 * a;
        let d2 = (-b + disc.sqrt()) / 2.0 * a;
        Intersections {
            count: 2,
            i1: Intersection {
                t: d1,
                object: self,
            },
            i2: Intersection {
                t: d2,
                object: self,
            },
        }
    }
}

impl<'a> Index<usize> for Intersections<'a> {
    type Output = Intersection<'a>;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.i1,
            1 => &self.i2,
            _ => panic!("Index out of range."),
        }
    }
}

impl<'a> Intersections<'a> {
    pub(crate) fn new(count: u8, i1: Intersection<'a>, i2: Intersection<'a>) -> Self {
        Self { count, i1, i2 }
    }

    pub(crate) fn hit(&self) -> Option<&Intersection> {
        if self.i1.t.is_sign_positive() && self.i2.t.is_sign_positive() {
            if self.i2.t > self.i1.t {
                Some(&self.i1)
            } else {
                Some(&self.i2)
            }
        } else if self.i1.t.is_sign_negative() && self.i2.t.is_sign_negative() {
            None
        } else if self.i1.t.is_sign_negative() {
            Some(&self.i2)
        } else if self.i2.t.is_sign_negative() {
            Some(&self.i1)
        } else {
            None
        }
    }
}
