use crate::{
    body::Body,
    color::Color,
    computed_intersection::ComputedIntersection,
    consts::EPSILON,
    intersections::Intersections,
    material::{Material, Phong},
    matrix::Matrix,
    point_light::PointLight,
    ray::Ray,
    sphere::Sphere,
    tuple::Tuple,
};

#[derive(Default, Debug, Clone)]
pub struct World {
    pub point_lights: Vec<PointLight>,
    pub bodies: Vec<Body>,
}

impl World {
    pub fn new(point_lights: Vec<PointLight>, bodies: Vec<Body>) -> Self {
        Self {
            point_lights,
            bodies,
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
        xs.sort_by(|a, b| (a.t.partial_cmp(&b.t)).unwrap());
        xs
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
    /// let c = world.shade_hit(comps);
    /// assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    ///
    /// world.point_lights = vec![PointLight::new(Tuple::Point(0.0, 0.25, 0.0), Color::new(1.0, 1.0, 1.0))];
    /// let ray = Ray::new(Tuple::Point(0, 0, 0), Tuple::Vector(0, 0, 1));
    /// let shape = world.bodies[1].clone();
    /// let i = Intersection::new(0.5, shape.into(), ray);
    /// let comps = i.to_computed();
    /// let c = world.shade_hit(comps);
    /// assert_eq!(c, Color::new(0.90498, 0.90498, 0.90498));
    /// ```
    pub fn shade_hit(&self, comps: ComputedIntersection) -> Color {
        assert_eq!(self.point_lights.len(), 1, "please read FIXME in shade_hit");
        let over_point = comps.point + comps.normalv * EPSILON;
        comps.body.material().lighting(
            // FIXME: why point_lights[0] is hard coded
            // maybe, iterate through all point lights and add the color of each light
            // adding might be a problem, if its sum > 1 for a color component
            self.point_lights[0].clone(),
            comps.point,
            comps.eyev,
            comps.normalv,
            self.is_shadowed(over_point),
        )
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
    pub fn color_at(&self, r: Ray) -> Color {
        let xs = self.intersect(r);
        if let Some(intersection) = xs.hit() {
            let cs = (*intersection).to_computed();
            self.shade_hit(cs)
        } else {
            Color::black()
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

    pub fn default_from_book() -> Self {
        Self {
            point_lights: vec![PointLight::new(
                Tuple::Point(-10.0, 10.0, -10.0),
                Color::new(1.0, 1.0, 1.0),
            )],
            bodies: vec![
                Sphere::default()
                    .with_material(Material::Phong(Phong {
                        color: Color::new(0.8, 1.0, 0.6),
                        diffuse: 0.7,
                        specular: 0.2,
                        ..Default::default()
                    }))
                    .into(),
                Sphere::default()
                    .with_transform(Matrix::Scaling(0.5, 0.5, 0.5))
                    .into(),
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
                expected = p.color;
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
}
