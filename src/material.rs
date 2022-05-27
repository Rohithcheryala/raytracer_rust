use crate::{
    body::Body,
    color::Color,
    matrix::Matrix,
    pattern::{Pattern, Stencil},
    point_light::PointLight,
    tuple::Tuple,
};

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Phong(Phong),
}

#[derive(Debug, Clone, Copy)]
pub struct Phong {
    // pub color: Color,
    pub pattern: Pattern,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
}

pub trait PhongLighting {
    fn lighting(
        &self,
        body: &Body,
        light: PointLight,
        position: Tuple,
        eyev: Tuple,
        normalv: Tuple,
        in_shadow: bool,
    ) -> Color;
}

impl PhongLighting for Material {
    fn lighting(
        &self,
        body: &Body,
        light: PointLight,
        position: Tuple,
        eyev: Tuple,
        normalv: Tuple,
        in_shadow: bool,
    ) -> Color {
        match self {
            Material::Phong(phong) => {
                phong.lighting(body, light, position, eyev, normalv, in_shadow)
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
}

impl Default for Phong {
    fn default() -> Self {
        Self {
            pattern: Pattern::default(),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}

impl PhongLighting for Phong {
    fn lighting(
        &self,
        body: &Body,
        light: PointLight,
        position: Tuple,
        eyev: Tuple,
        normalv: Tuple,
        in_shadow: bool,
    ) -> Color {
        let effective_color = self.color_at(body, position) * light.intensity;
        let ambient = effective_color * self.ambient;

        let lightv = (light.position - position).normalize();
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
        // HACK: why use branching here
        // ambient + (diffuse + specular) * in_shadow as u8;
        if in_shadow {
            ambient
        } else {
            ambient + diffuse + specular
        }
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
