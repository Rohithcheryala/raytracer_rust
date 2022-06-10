use crate::{
    body::Body,
    color::Color,
    computed_intersection::ComputedIntersection,
    intersections::Intersections,
    material::{Material, Phong, PhongLighting, Reflective, Refractive},
    matrix::Matrix,
    pattern::{Flat, Pattern},
    point_light::PointLight,
    ray::Ray,
    sphere::Sphere,
    tuple::Tuple,
};

#[derive(Default, Debug, Clone)]
pub struct World {
    pub point_lights: Vec<PointLight>,
    pub bodies: Vec<Body>,
    pub reflection_limit: usize,
}

impl World {
    pub fn new(point_lights: Vec<PointLight>, bodies: Vec<Body>, reflection_limit: usize) -> Self {
        Self {
            point_lights,
            bodies,
            reflection_limit,
        }
    }

    pub fn add_point_light(&mut self, l: PointLight) {
        self.point_lights.push(l);
    }

    pub fn add_body(&mut self, s: Body) {
        self.bodies.push(s);
    }

    pub fn intersect(&self, r: Ray) -> Intersections {
        let mut xs = Intersections::default();
        self.bodies.iter().for_each(|s| {
            let x = s.intersect(&r);
            xs.extend(x);
        });
        xs.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        xs
    }

    /// ```
    /// use raytracer_rust::world::World;
    /// use raytracer_rust::tuple::Tuple;
    /// use raytracer_rust::ray::Ray;
    /// use raytracer_rust::color::Color;
    ///
    /// let mut world = World::default_from_book();
    /// let ray = Ray::new(Tuple::Point(0, 0, -5), Tuple::Vector(0, 0, 1));
    /// let c = world.color_at(ray);
    /// assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    /// ```
    pub fn color_at(&self, ray: Ray) -> Color {
        self.color_at_with_reflection_limit(ray, self.reflection_limit)
    }

    fn color_at_with_reflection_limit(&self, ray: Ray, remaining_reflections: usize) -> Color {
        let xs = self.intersect(ray);
        if let Some(intersection) = xs.hit() {
            let (mu_from, mu_to) = xs.get_mu_shift(intersection);
            let material = intersection.body.material();
            let cs = intersection.to_computed(mu_from, mu_to);
            let surface_color = self.surface_color_at(&cs);
            let reflected_color = self.reflected_color_at(&cs, material, remaining_reflections);
            let refracted_color = self.refracted_color_at(&cs, material, remaining_reflections);
            let reflectance = cs.schlick();
            let total_color = if material.transparency() != 0.0 && material.reflectiveness() != 0.0
            {
                surface_color
                    + reflected_color * reflectance
                    + refracted_color * (1.0 - reflectance)
            } else {
                surface_color + reflected_color + refracted_color
            };
            total_color
        } else {
            Color::BLACK()
        }
    }

    /// ```
    /// use raytracer_rust::world::World;
    /// use raytracer_rust::computed_intersection::ComputedIntersection;
    /// use raytracer_rust::sphere::Sphere;
    /// use raytracer_rust::intersections::Intersection;
    /// use raytracer_rust::tuple::Tuple;
    /// use raytracer_rust::ray::Ray;
    /// use raytracer_rust::color::Color;
    /// use raytracer_rust::point_light::PointLight;
    ///
    /// let mut world = World::default_from_book();
    /// let ray = Ray::new(Tuple::Point(0, 0, -5), Tuple::Vector(0, 0, 1));
    ///
    /// let shape = world.bodies[0].clone();
    /// let i = Intersection::new(4.0, shape.into(), ray);
    /// let comps = i.to_computed();
    /// let c = world.surface_color_at(&comps);
    /// assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    ///
    /// world.point_lights = vec![PointLight::new(Tuple::Point(0.0, 0.25, 0.0), Color::new(1.0, 1.0, 1.0))];
    /// let ray = Ray::new(Tuple::Point(0, 0, 0), Tuple::Vector(0, 0, 1));
    /// let shape = world.bodies[1].clone();
    /// let i = Intersection::new(0.5, shape.into(), ray);
    /// let comps = i.to_computed();
    /// let c = world.surface_color_at(&comps);
    /// assert_eq!(c, Color::new(0.90498, 0.90498, 0.90498));
    /// ```
    pub fn surface_color_at(&self, comps: &ComputedIntersection) -> Color {
        assert_eq!(
            self.point_lights.len(),
            1,
            "please read FIXME in surface_color_at"
        );
        let surface_color = comps.body.material().lighting(
            // FIXME: why point_lights[0] is hard coded
            // maybe, iterate through all point lights and add the color of each light
            // adding might be a problem, if its sum > 1 for a color component
            &comps.body,
            self.point_lights[0],
            comps.point,
            comps.eyev,
            comps.normalv,
            self.transparency_factor(comps.over_point),
        );
        surface_color
    }

    fn reflected_color_at(
        &self,
        cs: &ComputedIntersection,
        material: &Material,
        remaining_reflections: usize,
    ) -> Color {
        if remaining_reflections > 0 && material.reflectiveness() != 0.0 {
            let reflected_ray = Ray::new(cs.over_point, cs.reflectv);
            let color =
                self.color_at_with_reflection_limit(reflected_ray, remaining_reflections - 1);
            color * material.reflectiveness()
        } else {
            Color::BLACK()
        }
    }

    fn refracted_color_at(
        &self,
        cs: &ComputedIntersection,
        material: &Material,
        remaining_reflections: usize,
    ) -> Color {
        if remaining_reflections > 0 && material.transparency() != 0.0 {
            let mu_ratio = cs.mu_from / cs.mu_to;
            let cos_i = cs.eyev.dot(&cs.normalv);
            let sin2_t = (mu_ratio * mu_ratio) as f64 * (1.0 - (cos_i * cos_i));
            if sin2_t <= 1.0 {
                // refract
                let cos_t = (1.0 - sin2_t).sqrt();
                let direction = cs.normalv * (mu_ratio as f64 * cos_i - cos_t) - cs.eyev * mu_ratio;

                let refracted_ray = Ray::new(cs.under_point, direction);
                let refracted_color =
                    self.color_at_with_reflection_limit(refracted_ray, remaining_reflections - 1);

                refracted_color * material.transparency()
            } else {
                // total-internal reflection
                Color::BLACK()
            }
        } else {
            Color::BLACK()
        }
    }

    // FIXME: using "any" is not well understood.
    pub fn is_shadowed(&self, point: Tuple) -> bool {
        self.point_lights.iter().any(|light| {
            let v = light.position - point;
            let distance = v.magnitude();
            let direction = v.normalize();
            let r = Ray::new(point, direction);
            let intersections = self.intersect(r);
            intersections.hit().map(|i| i.t < distance).unwrap_or(false)
        })
    }

    pub fn transparency_factor(&self, point: Tuple) -> f64 {
        let light = self.point_lights[0].clone();
        let v = light.position - point;
        let distance = v.magnitude();
        let direction = v.normalize();
        let r = Ray::new(point, direction);
        let intersections = self.intersect(r);
        let mut hit_objects = vec![];
        let factor = intersections
            .iter()
            .filter(|i| !(i.t.is_sign_negative() || i.t > distance))
            .fold(1.0, |acc, i| {
                if hit_objects.contains(&i.body.material()) {
                    acc
                } else {
                    hit_objects.push(i.body.material());
                    acc * i.body.material().transparency() as f64
                }
            });
        factor
    }

    pub fn default_from_book() -> Self {
        Self {
            point_lights: vec![PointLight::new(
                Tuple::Point(-10.0, 10.0, -10.0),
                Color::new(1.0, 1.0, 1.0),
            )],
            bodies: vec![
                Sphere::default()
                    .with_material(Material::Phong(Phong {
                        pattern: Pattern::Flat(Flat::new(Color::new(0.8, 1.0, 0.6))),
                        diffuse: 0.7,
                        specular: 0.2,
                        ..Default::default()
                    }))
                    .into(),
                Sphere::default()
                    .with_transform(Matrix::Scaling(0.5, 0.5, 0.5))
                    .into(),
            ],
            reflection_limit: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pattern::Stencil;

    #[test]
    fn test_intersect_world_with_ray() {
        let w = World::default_from_book();
        let r = Ray::new(Tuple::Point(0.0, 0.0, -5.0), Tuple::Vector(0.0, 0.0, 1.0));
        let xs = w.intersect(r);
        assert_eq!(xs.count(), 4);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 4.5);
        assert_eq!(xs[2].t, 5.5);
        assert_eq!(xs[3].t, 6.0);
    }

    #[test]
    fn test_intersect_world_behind_ray() {
        let mut w = World::default_from_book();
        match w.bodies[0].material_mut() {
            Material::Phong(p) => p.ambient = 1.0,
        }
        let expected;
        match w.bodies[1].material_mut() {
            Material::Phong(p) => {
                p.ambient = 1.0;
                expected = p.color_at_in_pattern_space(Tuple::Point(0, 0, 0));
            }
        }
        let r = Ray::new(Tuple::Point(0.0, 0.0, 0.75), Tuple::Vector(0.0, 0.0, -1.0));
        let c = w.color_at(r);
        assert_eq!(c, expected);
    }

    #[test]
    fn is_shadowed_test() {
        let w = World::default_from_book();
        assert_eq!(w.is_shadowed(Tuple::Point(0.0, 10.0, 0.0)), false);
        assert_eq!(w.is_shadowed(Tuple::Point(10.0, -10.0, 10.0)), true);
        assert_eq!(w.is_shadowed(Tuple::Point(-20.0, 20.0, -20.0)), false);
        assert_eq!(w.is_shadowed(Tuple::Point(-2.0, 2.0, -2.0)), false);
    }

    #[test]
    fn transparency_factor_works() {
        let s1 = Sphere::new(
            Matrix::Identity(),
            Material::Phong(Phong {
                transparency: 0.5,
                ..Default::default()
            }),
        );
        let s2 = Sphere::new(
            Matrix::Translation(10, 0, 0),
            Material::Phong(Phong {
                transparency: 0.25,
                ..Default::default()
            }),
        );
        let w = World::new(
            vec![PointLight::new(
                Tuple::Point(-100.0, 0.0, 0.0),
                Color::WHITE(),
            )],
            vec![s1.into(), s2.into()],
            5,
        );
        let result = w.transparency_factor(Tuple::Point(100, 0, 0));
        assert_eq!(result, 0.5 * 0.25);
    }
}
