use crate::{body::Body, intersections::Intersection, tuple::Tuple};
pub struct ComputedIntersection {
    pub inside: bool,
    pub point: Tuple,
    pub body: Body,
    pub eyev: Tuple,
    pub normalv: Tuple,
}

impl ComputedIntersection {
    pub fn new(inside: bool, point: Tuple, body: Body, eyev: Tuple, normalv: Tuple) -> Self {
        Self {
            inside,
            point,
            body,
            eyev,
            normalv,
        }
    }
}

impl From<Intersection> for ComputedIntersection {
    fn from(intersection: Intersection) -> Self {
        intersection.to_computed()
    }
}
