use crate::tuple::Tuple;

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
}
