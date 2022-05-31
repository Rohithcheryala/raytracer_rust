use crate::{body::Body, intersections::Intersection, tuple::Tuple};
pub struct ComputedIntersection {
    pub inside: bool,
    pub point: Tuple,
    pub over_point: Tuple,
    pub body: Body,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub reflectv: Tuple,
}

impl ComputedIntersection {
    pub fn new(
        inside: bool,
        point: Tuple,
        over_point: Tuple,
        body: Body,
        eyev: Tuple,
        normalv: Tuple,
        reflectv: Tuple,
    ) -> Self {
        Self {
            inside,
            point,
            over_point,
            body,
            eyev,
            normalv,
            reflectv,
        }
    }
}

impl From<Intersection> for ComputedIntersection {
    fn from(intersection: Intersection) -> Self {
        intersection.to_computed()
    }
}
