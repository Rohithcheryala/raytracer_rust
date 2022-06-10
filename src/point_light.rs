use crate::{color::Color, tuple::Tuple};

#[derive(Debug, Clone, Copy)]
pub struct PointLight {
    pub position: Tuple, // Point
    pub intensity: Color,
}

impl PointLight {
    #[inline]
    pub fn new(position: Tuple, intensity: Color) -> Self {
        Self {
            position,
            intensity,
        }
    }
}
