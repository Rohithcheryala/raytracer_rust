use crate::{color::Color, point_light::PointLight, tuple::Tuple};

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Phong(Phong),
}

#[derive(Debug, Clone, Copy)]
pub struct Phong {
    pub color: Color,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
}

pub trait PhongLightning {
    fn lightning(&self, light: PointLight, point: Tuple, eyev: Tuple, normalv: Tuple) -> Color;
}

impl Material {
    pub fn lighting(&self, light: PointLight, point: Tuple, eyev: Tuple, normalv: Tuple) -> Color {
        match self {
            Material::Phong(phong) => phong.lightning(light, point, eyev, normalv),
        }
    }
}

impl Phong {
    pub fn with_color(mut self, c: Color) -> Self {
        self.color = c;
        self
    }

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
}

impl Default for Phong {
    fn default() -> Self {
        Self {
            color: Color::default(),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}

impl PhongLightning for Phong {
    fn lightning(&self, light: PointLight, point: Tuple, eyev: Tuple, normalv: Tuple) -> Color {
        let effective_color = self.color * light.intensity;
        let ambient = effective_color * self.ambient;

        let lightv = (light.position - point).normalize();
        let light_dot_normal = lightv.dot(&normalv);
        let (diffuse, specular);

        if light_dot_normal < 0.0 {
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
