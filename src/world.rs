use crate::{
    color::Color,
    computed_intersection::ComputedIntersection,
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
    pub spheres: Vec<Sphere>,
}

impl World {
    pub fn new() -> Self {
        Self {
            point_lights: Vec::new(),
            spheres: Vec::new(),
        }
    }

    pub fn add_point_light(&mut self, l: PointLight) {
        self.point_lights.push(l);
    }

    pub fn add_sphere(&mut self, s: Sphere) {
        self.spheres.push(s);
    }

    pub fn intersect(&self, r: Ray) -> Intersections {
        let mut xs = Intersections::default();
        self.spheres.iter().for_each(|s| {
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
    /// let shape = world.spheres[0].clone();
    /// let i = Intersection::new(4.0, shape, ray);
    /// let comps = i.to_computed();
    /// let c = world.shade_hit(comps);
    /// assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    ///
    /// world.point_lights = vec![PointLight::new(Tuple::Point(0.0, 0.25, 0.0), Color::new(1.0, 1.0, 1.0))];
    /// let ray = Ray::new(Tuple::Point(0, 0, 0), Tuple::Vector(0, 0, 1));
    /// let shape = world.spheres[1].clone();
    /// let i = Intersection::new(0.5, shape, ray);
    /// let comps = i.to_computed();
    /// let c = world.shade_hit(comps);
    /// assert_eq!(c, Color::new(0.90498, 0.90498, 0.90498));
    /// ```
    pub fn shade_hit(&self, comps: ComputedIntersection) -> Color {
        comps.object.material.lighting(
            self.point_lights[0].clone(),
            comps.point,
            comps.eyev,
            comps.normalv,
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

    pub fn default_from_book() -> Self {
        Self {
            point_lights: vec![PointLight::new(
                Tuple::Point(-10.0, 10.0, -10.0),
                Color::new(1.0, 1.0, 1.0),
            )],
            spheres: vec![
                Sphere::default().with_material(Material::Phong(Phong {
                    color: Color::new(0.8, 1.0, 0.6),
                    diffuse: 0.7,
                    specular: 0.2,
                    ..Default::default()
                })),
                Sphere::default().with_transform(Matrix::Scaling(0.5, 0.5, 0.5)),
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
        match &mut w.spheres[0].material {
            Material::Phong(p) => p.ambient = 1.0,
        }
        let expected;
        match &mut w.spheres[1].material {
            Material::Phong(p) => {
                p.ambient = 1.0;
                expected = p.color;
            }
        }
        let r = Ray::new(Tuple::Point(0.0, 0.0, 0.75), Tuple::Vector(0.0, 0.0, -1.0));
        let c = w.color_at(r);
        assert_eq!(c, expected);
    }
}
