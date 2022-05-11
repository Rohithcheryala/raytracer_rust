use crate::{intersections::Intersection, sphere::Sphere, tuple::Tuple};
pub struct ComputedIntersection {
    pub inside: bool,
    pub point: Tuple,
    pub object: Sphere,
    pub eyev: Tuple,
    pub normalv: Tuple,
}

impl ComputedIntersection {
    pub fn new(inside: bool, point: Tuple, object: Sphere, eyev: Tuple, normalv: Tuple) -> Self {
        Self {
            inside,
            point,
            object,
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
