use crate::color::Color;
use crate::color::ToRGB;

pub(crate) trait Sized {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
}

pub(crate) trait ToPPM: Sized {
    fn header(&self) -> String {
        let mut output = String::new();
        output += format!("P3\n{} {}\n255\n", self.width(), self.height()).as_str();
        output
    }
    fn to_ppm(&self) -> String;
    fn save_as_ppm(&self, save_to: String) -> Result<(), String> {
        let mut out = self.header();
        out.push_str(self.to_ppm().as_str());
        std::fs::write(save_to, out).unwrap();
        Ok(())
    }
}

pub(crate) struct Canvas {
    width: usize,
    height: usize,
    frame: Vec<Vec<Color>>,
}

impl Canvas {
    pub(crate) fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            frame: vec![vec![Color::black(); width]; height],
        }
    }

    pub(crate) fn set_color_at_pixel(&mut self, x: f64, y: f64, color: Color) {
        self.frame[y as usize][x as usize] = color;
    }
}

impl Sized for Canvas {
    fn width(&self) -> usize {
        self.width
    }
    fn height(&self) -> usize {
        self.height
    }
}

impl ToPPM for Canvas {
    fn to_ppm(&self) -> String {
        let mut output = String::new();

        for j in 0..(self.height as i64) {
            for i in 0..(self.width as i64) {
                output +=
                    format!("{}\n", self.frame[j as usize][i as usize].to_rgb_string()).as_str();
            }
        }

        output
    }
}
