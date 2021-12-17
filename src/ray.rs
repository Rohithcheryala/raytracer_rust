use crate::{matrix::Matrix2D, tuple::Tuple};

#[derive(Clone, Copy, Debug)]
pub(crate) struct Ray {
    pub(crate) origin: Tuple,    // basically Point
    pub(crate) direction: Tuple, // basically Vector
}

impl Ray {
    pub(crate) fn new(origin: Tuple, direction: Tuple) -> Self {
        Self { origin, direction }
    }
    pub(crate) fn position(&self, t: f64) -> Tuple {
        self.origin + self.direction * t
    }
    pub(crate) fn transform(&self, t: Matrix2D) -> Self {
        Self {
            origin: &t * self.origin,
            direction: &t * self.direction,
        }
    }
    pub(crate) fn as_tuple(&self) -> (Tuple, Tuple) {
        (self.origin, self.direction)
    }
}
