use crate::{
    body::Body,
    color::Color,
    matrix::Matrix,
    pattern::{Pattern, Stencil},
    point_light::PointLight,
    tuple::Tuple,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Material {
    Phong(Phong),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Phong {
    // pub color: Color,
    pub pattern: Pattern,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
    pub reflectiveness: f32,
    pub transparency: f32,
    pub refractive_index: f32,
}

pub trait Reflective {
    fn reflectiveness(&self) -> f32;
}

impl Reflective for Material {
    fn reflectiveness(&self) -> f32 {
        match self {
            Material::Phong(p) => p.reflectiveness,
        }
    }
}

pub trait Refractive {
    fn refractive_index(&self) -> f32;
    fn transparency(&self) -> f32;
}

impl Refractive for Material {
    fn refractive_index(&self) -> f32 {
        match self {
            Material::Phong(p) => p.refractive_index,
        }
    }

    fn transparency(&self) -> f32 {
        match self {
            Material::Phong(p) => p.transparency,
        }
    }
}

pub trait PhongLighting {
    fn lighting(
        &self,
        body: &Body,
        light: PointLight,
        point: Tuple,
        eyev: Tuple,
        normalv: Tuple,
        transparency_factor: f64,
    ) -> Color;
}

impl PhongLighting for Material {
    fn lighting(
        &self,
        body: &Body,
        light: PointLight,
        point: Tuple,
        eyev: Tuple,
        normalv: Tuple,
        transparency_factor: f64,
    ) -> Color {
        match self {
            Material::Phong(phong) => {
                phong.lighting(body, light, point, eyev, normalv, transparency_factor)
            }
        }
    }
}

impl Phong {
    pub fn with_ambient(mut self, c: f32) -> Self {
        self.ambient = c;
        self
    }

    pub fn with_diffuse(mut self, c: f32) -> Self {
        self.diffuse = c;
        self
    }

    pub fn with_specular(mut self, c: f32) -> Self {
        self.specular = c;
        self
    }

    pub fn with_shininess(mut self, c: f32) -> Self {
        self.shininess = c;
        self
    }

    pub fn with_reflectiveness(mut self, c: f32) -> Self {
        self.reflectiveness = c;
        self
    }
}

impl Default for Phong {
    fn default() -> Self {
        Self {
            pattern: Pattern::default(),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            reflectiveness: 0.0,
            transparency: 0.0,
            refractive_index: 1.0,
        }
    }
}

impl PhongLighting for Phong {
    fn lighting(
        &self,
        body: &Body,
        light: PointLight,
        point: Tuple,
        eyev: Tuple,
        normalv: Tuple,
        transparency_factor: f64,
    ) -> Color {
        let effective_color = self.color_at(body, point) * light.intensity;
        let ambient = effective_color * self.ambient;

        let lightv = (light.position - point).normalize();
        let light_dot_normal = lightv.dot(&normalv);
        let (diffuse, specular);

        if light_dot_normal < 0.0 {
            diffuse = Color::BLACK();
            specular = Color::BLACK();
        } else {
            diffuse = effective_color * self.diffuse * light_dot_normal;
            let reflectv = (-lightv).reflect(normalv);
            let reflect_dot_eye = reflectv.dot(&eyev);
            if reflect_dot_eye <= 0f64 {
                specular = Color::BLACK();
            } else {
                let factor = reflect_dot_eye.powf(self.shininess as f64);
                specular = light.intensity * self.specular * factor;
            }
        }
        ambient + (diffuse + specular) * transparency_factor
    }
}

impl Stencil for Phong {
    fn color_at_in_pattern_space(&self, position: Tuple) -> Color {
        self.pattern.color_at_in_pattern_space(position)
    }

    fn transform(&self) -> Matrix<4> {
        self.pattern.transform()
    }
}

impl From<Phong> for Material {
    fn from(phong: Phong) -> Self {
        Material::Phong(phong)
    }
}
