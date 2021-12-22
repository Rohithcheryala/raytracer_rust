use crate::{color::Color, tuple::Tuple};

#[derive(Debug)]
pub(crate) struct PointLight {
    pub(crate) intensity: Color,
    pub(crate) position: Tuple, // Point
}

impl PointLight {
    pub(crate) fn new(intensity: Color, position: Tuple) -> Self {
        Self {
            intensity,
            position,
        }
    }
}
