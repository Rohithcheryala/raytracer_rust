use crate::{color::Color, tuple::Tuple};

#[derive(Debug)]
pub(crate) struct PointLight {
    pub(crate) position: Tuple, // Point
    pub(crate) intensity: Color,
}

impl PointLight {
    #[inline]
    pub(crate) fn new(position: Tuple, intensity: Color) -> Self {
        Self {
            position,
            intensity,
        }
    }
}
