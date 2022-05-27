use crate::{
    body::{Body, Intersectable},
    color::Color,
    matrix::Matrix,
    tuple::Tuple,
};

pub trait Stencil {
    fn color_at(&self, body: &Body, world_point: Tuple) -> Color {
        let object_point = body.transform().inverse() * world_point;
        let pattern_point = self.transform().inverse() * object_point;

        self.color_at_in_pattern_space(pattern_point)
    }
    fn transform(&self) -> Matrix<4>;
    fn color_at_in_pattern_space(&self, position: Tuple) -> Color;
}

#[derive(Clone, Copy, Debug)]
pub enum Pattern {
    Checkers(Checkers),
    Flat(Flat),
    Gradient(Gradient),
    Ring(Ring),
    Striped(Striped),
}

#[derive(Clone, Copy, Debug)]
pub struct Striped {
    color_a: Color,
    color_b: Color,
    pub transform: Matrix<4>,
}

#[derive(Clone, Copy, Debug)]
pub struct Flat {
    color: Color,
    pub transform: Matrix<4>,
}

#[derive(Clone, Copy, Debug)]
pub struct Gradient {
    color_a: Color,
    color_b: Color,
    pub transform: Matrix<4>,
}

#[derive(Clone, Copy, Debug)]
pub struct Checkers {
    color_a: Color,
    color_b: Color,
    pub transform: Matrix<4>,
}

#[derive(Clone, Copy, Debug)]
pub struct Ring {
    color_a: Color,
    color_b: Color,
    pub transform: Matrix<4>,
}

impl Striped {
    pub fn new(color_a: Color, color_b: Color, transform: Matrix<4>) -> Self {
        Self {
            color_a,
            color_b,
            transform,
        }
    }
}

impl Flat {
    pub fn new(color: Color) -> Self {
        Self {
            color,
            transform: Matrix::Identity(),
        }
    }
}

impl Gradient {
    pub fn new(color_a: Color, color_b: Color, transform: Matrix<4>) -> Self {
        Self {
            color_a,
            color_b,
            transform,
        }
    }
}

impl Ring {
    pub fn new(color_a: Color, color_b: Color, transform: Matrix<4>) -> Self {
        Self {
            color_a,
            color_b,
            transform,
        }
    }
}

impl Checkers {
    pub fn new(color_a: Color, color_b: Color, transform: Matrix<4>) -> Self {
        Self {
            color_a,
            color_b,
            transform,
        }
    }
}

impl Stencil for Pattern {
    fn color_at_in_pattern_space(&self, position: Tuple) -> Color {
        match self {
            Pattern::Flat(flat) => flat.color_at_in_pattern_space(position),
            Pattern::Striped(stripe) => stripe.color_at_in_pattern_space(position),
            Pattern::Gradient(gradient) => gradient.color_at_in_pattern_space(position),
            Pattern::Ring(ring) => ring.color_at_in_pattern_space(position),
            Pattern::Checkers(checkers) => checkers.color_at_in_pattern_space(position),
        }
    }
    fn transform(&self) -> Matrix<4> {
        match self {
            Pattern::Flat(flat) => flat.transform(),
            Pattern::Striped(stripe) => stripe.transform(),
            Pattern::Gradient(gradient) => gradient.transform(),
            Pattern::Ring(ring) => ring.transform(),
            Pattern::Checkers(checkers) => checkers.transform(),
        }
    }
}

impl Stencil for Flat {
    fn color_at_in_pattern_space(&self, _position: Tuple) -> Color {
        self.color
    }

    fn transform(&self) -> Matrix<4> {
        self.transform
    }
}

impl Stencil for Striped {
    fn color_at_in_pattern_space(&self, position: Tuple) -> Color {
        // if (position.x % self.width as f64).floor() == 0.0 {
        if (position.x.floor() as isize % 2) == 0 {
            self.color_a
        } else {
            self.color_b
        }
    }
    fn transform(&self) -> Matrix<4> {
        self.transform
    }
}

impl Stencil for Gradient {
    fn color_at_in_pattern_space(&self, position: Tuple) -> Color {
        self.color_a + (self.color_b - self.color_a) * (position.x - position.x.floor())
        // self.color_a * (todo!()) + self.color_b * (todo!())
    }
    fn transform(&self) -> Matrix<4> {
        self.transform
    }
}

impl Stencil for Ring {
    fn color_at_in_pattern_space(&self, position: Tuple) -> Color {
        let distance_from_center = (position.x.powi(2) + position.y.powi(2)).sqrt();
        if distance_from_center.floor() as isize % 2 == 0 {
            self.color_a
        } else {
            self.color_b
        }
    }
    fn transform(&self) -> Matrix<4> {
        self.transform
    }
}

impl Stencil for Checkers {
    fn color_at_in_pattern_space(&self, position: Tuple) -> Color {
        let distance = position.x.floor() + position.y.floor() + position.z.floor();
        if distance as isize % 2 == 0 {
            self.color_a
        } else {
            self.color_b
        }
    }
    fn transform(&self) -> Matrix<4> {
        self.transform
    }
}

impl Default for Pattern {
    fn default() -> Self {
        Pattern::Flat(Flat::default())
    }
}

impl Default for Flat {
    fn default() -> Self {
        Self {
            color: Color::default(),
            transform: Matrix::Identity(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::sphere::Sphere;

    use super::*;

    #[test]
    fn stripe_works() {
        let stripe = Striped {
            color_a: Color::WHITE(),
            color_b: Color::BLACK(),
            transform: Matrix::Identity(),
        };
        assert_eq!(
            stripe.color_at_in_pattern_space(Tuple::Point(0.0, 0.0, 0.0)),
            Color::WHITE()
        );
        assert_eq!(
            stripe.color_at_in_pattern_space(Tuple::Point(0.0, 1.0, 0.0)),
            Color::WHITE()
        );
        assert_eq!(
            stripe.color_at_in_pattern_space(Tuple::Point(0.0, 2.0, 0.0)),
            Color::WHITE()
        );

        assert_eq!(
            stripe.color_at_in_pattern_space(Tuple::Point(0.0, 0.0, 0.0)),
            Color::WHITE()
        );
        assert_eq!(
            stripe.color_at_in_pattern_space(Tuple::Point(0.0, 0.0, 1.0)),
            Color::WHITE()
        );
        assert_eq!(
            stripe.color_at_in_pattern_space(Tuple::Point(0.0, 0.0, 2.0)),
            Color::WHITE()
        );

        assert_eq!(
            stripe.color_at_in_pattern_space(Tuple::Point(0.0, 0.0, 0.0)),
            Color::WHITE()
        );
        assert_eq!(
            stripe.color_at_in_pattern_space(Tuple::Point(0.9, 0.0, 0.0)),
            Color::WHITE()
        );
        assert_eq!(
            stripe.color_at_in_pattern_space(Tuple::Point(1.0, 0.0, 0.0)),
            Color::BLACK()
        );
        assert_eq!(
            stripe.color_at_in_pattern_space(Tuple::Point(-0.1, 0.0, 0.0)),
            Color::BLACK()
        );
        assert_eq!(
            stripe.color_at_in_pattern_space(Tuple::Point(-1.0, 0.0, 0.0)),
            Color::BLACK()
        );
        assert_eq!(
            stripe.color_at_in_pattern_space(Tuple::Point(-1.1, 0.0, 0.0)),
            Color::WHITE()
        );
    }

    #[test]
    fn striped_pattern_adheres_to_object_transform() {
        let transform = Matrix::Scaling(2.0, 2.0, 2.0);
        let pattern = Pattern::Striped(Striped {
            color_a: Color::BLACK(),
            color_b: Color::WHITE(),
            transform: Matrix::Identity(),
        });
        let body = Body::from(Sphere::default().with_transform(transform));

        assert_eq!(
            Color::BLACK(),
            pattern.color_at(&body, Tuple::Point(1.5, 0.0, 0.0),)
        );
    }
}
