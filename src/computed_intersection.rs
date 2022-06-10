use crate::{body::Body, tuple::Tuple};
pub struct ComputedIntersection {
    pub inside: bool,
    pub point: Tuple,
    pub over_point: Tuple,
    pub under_point: Tuple,
    pub body: Body,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub reflectv: Tuple,
    pub mu_from: f32,
    pub mu_to: f32,
}

impl ComputedIntersection {
    pub fn new(
        inside: bool,
        point: Tuple,
        over_point: Tuple,
        under_point: Tuple,
        body: Body,
        eyev: Tuple,
        normalv: Tuple,
        reflectv: Tuple,
        mu_from: f32,
        mu_to: f32,
    ) -> Self {
        Self {
            inside,
            point,
            over_point,
            under_point,
            body,
            eyev,
            normalv,
            reflectv,
            mu_from,
            mu_to,
        }
    }

    pub fn schlick(&self) -> f64 {
        let mut cos_i = self.eyev.dot(&self.normalv);
        if self.mu_from > self.mu_to {
            let n = self.mu_from / self.mu_to;
            let sin2_t = (n * n) as f64 * (1.0 - cos_i * cos_i);
            if sin2_t > 1.0 {
                return 1.0;
            }
            let cos_t = (1.0 - sin2_t).sqrt();
            cos_i = cos_t;
        }
        let r0 = ((self.mu_from - self.mu_to) / (self.mu_from + self.mu_to)).powi(2) as f64;
        r0 + (1.0 - r0) * (1.0 - cos_i).powi(5)
    }
}

// impl From<Intersection> for ComputedIntersection {
//     fn from(intersection: Intersection) -> Self {
//         intersection.to_computed()
//     }
// }
