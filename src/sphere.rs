use super::Ray;
use crate::{
    color::Color,
    matrix::Matrix,
    point_light::PointLight,
    tuple::{Point, Tuple},
};
use std::ops::Index;

#[derive(Debug, Clone)]
pub(crate) struct Sphere {
    pub(crate) transform: Matrix,
    pub(crate) material: Material,
}

#[derive(Debug, Clone)]
pub(crate) struct Phong {
    pub(crate) color: Color,
    ambient: f32,
    diffuse: f32,
    specular: f32,
    shininess: f32,
}

pub(crate) trait PhongLightning {
    fn lightning(&self, light: PointLight, point: Tuple, eyev: Tuple, normalv: Tuple) -> Color;
}

#[derive(Debug, Clone)]
pub(crate) enum Material {
    Phong(Phong),
}

#[derive(Debug)]
pub(crate) struct Intersections<'a> {
    count: u8,
    i1: Intersection<'a>,
    i2: Intersection<'a>,
}

#[derive(Debug)]
pub(crate) struct Intersection<'a> {
    pub(crate) t: f64,
    pub(crate) object: &'a Sphere,
}

impl Sphere {
    pub(crate) fn new() -> Self {
        Self {
            transform: Matrix::Identity(),
            material: Material::default(),
        }
    }

    pub(crate) fn set_transform(&mut self, t: Matrix) {
        self.transform = t;
    }

    pub(crate) fn intersect(&self, r: Ray) -> Intersections {
        let r = r.transform(self.transform.inverse());
        let sphere_to_ray = r.origin() - Point::new(0.0, 0.0, 0.0);
        // origin: (Ox, Oy,Oz)
        // let (ox, oy, oz, _ow) = r.origin.as_tuple();
        // direction: (dx,dy,dz)
        // let (dx, dy, dz, _dw) = r.direction.as_tuple();
        // any point on ray: ( Ox+(dx*t), Oy+(dy*t), Oz+(dz*t) )
        // if distance of this point from sphere center is r then
        // this point lies on the sphere
        // eqn: x^2 + y^2 + z^2 = r^2 = 1^2
        // substituting and rearranging the terms gives
        // (dx^2+dy^2+dz^2)t^2 + 2(oxdx+oydy+ozdz)t + (ox^2+oy^2+oz^2-r^2) - 1 = 0
        let a = r.direction().dot(&r.direction()); // dx.powi(2) + dy.powi(2) + dz.powi(2);
        let b = 2.0 * r.direction().dot(&sphere_to_ray); // 2.0 * (ox * dx + oy * dy + oz * dz);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0; // ox.powi(2) + oy.powi(2) + oz.powi(2) - (self.r as f64).powi(2);

        let disc = b * b - 4.0 * a * c;
        let d1 = (-b - disc.sqrt()) / (2.0 * a);
        let d2 = (-b + disc.sqrt()) / (2.0 * a);
        Intersections {
            count: 2,
            i1: Intersection {
                t: d1,
                object: self,
            },
            i2: Intersection {
                t: d2,
                object: self,
            },
        }
    }

    pub(crate) fn normal_at(&self, world_point: Tuple) -> Tuple {
        let object_point = self.transform.inverse() * world_point;
        let object_normal = object_point - Point::new(0.0, 0.0, 0.0);
        let mut world_normal = self.transform.inverse() * object_normal;
        world_normal.w = 0.0;
        world_normal.normalize()
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for Material {
    fn default() -> Self {
        Material::Phong(Phong {
            color: Color::default(),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        })
    }
}

impl<'a> Index<usize> for Intersections<'a> {
    type Output = Intersection<'a>;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.i1,
            1 => &self.i2,
            _ => panic!("Index out of range."),
        }
    }
}

impl<'a> Intersections<'a> {
    pub(crate) fn new(count: u8, i1: Intersection<'a>, i2: Intersection<'a>) -> Self {
        Self { count, i1, i2 }
    }

    pub(crate) fn hit(&self) -> Option<&Intersection> {
        if self.i1.t.is_sign_positive() && self.i2.t.is_sign_positive() {
            if self.i2.t > self.i1.t {
                Some(&self.i1)
            } else {
                Some(&self.i2)
            }
        } else if self.i1.t.is_sign_negative() && self.i2.t.is_sign_negative() {
            None
        } else if self.i1.t.is_sign_negative() {
            Some(&self.i2)
        } else if self.i2.t.is_sign_negative() {
            Some(&self.i1)
        } else {
            None
        }
    }
}

impl Material {
    pub(crate) fn lighting(
        &self,
        light: PointLight,
        point: Tuple,
        eyev: Tuple,
        normalv: Tuple,
    ) -> Color {
        match self {
            Material::Phong(phong) => phong.lightning(light, point, eyev, normalv),
        }
    }
}

impl PhongLightning for Phong {
    fn lightning(&self, light: PointLight, point: Tuple, eyev: Tuple, normalv: Tuple) -> Color {
        let effective_color = self.color * light.intensity;
        let ambient = effective_color * self.ambient;

        let lightv = (light.position - point).normalize();
        let light_dot_normal = lightv.dot(&normalv);
        let diffuse;
        let specular;

        if light_dot_normal < 0f64 {
            diffuse = Color::black();
            specular = Color::black();
        } else {
            diffuse = effective_color * self.diffuse * light_dot_normal;
            let reflectv = (-lightv).reflect(normalv);
            let reflect_dot_eye = reflectv.dot(&eyev);
            if reflect_dot_eye <= 0f64 {
                specular = Color::black();
            } else {
                let factor = reflect_dot_eye.powf(self.shininess as f64);
                specular = light.intensity * self.specular * factor;
            }
        }
        ambient + diffuse + specular
    }
}
