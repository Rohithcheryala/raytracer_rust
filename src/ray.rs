use crate::{matrix::Matrix, tuple::Tuple};

#[derive(Clone, Copy, Debug)]
pub(crate) struct Ray {
    origin: Tuple,    // basically Point
    direction: Tuple, // basically Vector
}

impl Ray {
    #[inline]
    pub(crate) fn new(origin: Tuple, direction: Tuple) -> Self {
        Self { origin, direction }
    }

    pub(crate) fn origin(&self) -> Tuple {
        self.origin
    }

    pub(crate) fn direction(&self) -> Tuple {
        self.direction
    }

    pub(crate) fn position(&self, t: f64) -> Tuple {
        self.origin + self.direction * t
    }

    pub(crate) fn transform(&self, t: Matrix) -> Self {
        Self {
            origin: &t * self.origin,
            direction: &t * self.direction,
        }
    }

    pub(crate) fn as_tuple(&self) -> (Tuple, Tuple) {
        (self.origin, self.direction)
    }
}
