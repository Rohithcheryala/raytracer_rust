use crate::color::{Color, ToRGB};
use std::{convert::AsRef, fs, path::Path, slice::Iter};

pub trait Sized {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
}

pub trait ToPPM: Sized {
    // FIXME: why use String, not &str?
    fn header(&self) -> String {
        let mut output = String::new();
        output += format!("P3\n{} {}\n255\n", self.width(), self.height()).as_str();
        output
    }
    fn to_ppm(&self) -> String;
    fn save_as_ppm<T: AsRef<str>>(&self, save_to: T) -> std::io::Result<()> {
        let mut out = self.header();
        out.push_str(self.to_ppm().as_str());
        // TODO: read more about this
        // !https://stackoverflow.com/questions/59046312/how-can-i-create-a-file-and-its-parent-directories-using-a-single-method-in-rust
        let path = Path::new(save_to.as_ref());
        let prefix = path.parent().unwrap();
        fs::create_dir_all(prefix).unwrap();
        fs::write(path, out)
    }
}

pub struct Canvas {
    width: usize,
    height: usize,
    frame: Vec<Vec<Color>>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            frame: vec![vec![Color::black(); width]; height],
        }
    }

    pub fn set_color_at_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.frame[y][x] = color;
    }

    pub fn color_at_pixel(&self, x: usize, y: usize) -> Color {
        self.frame[y][x]
    }

    pub fn iter(&self) -> Iter<Vec<Color>> {
        self.frame.iter()
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
        // THEETA:
        let mut output = String::with_capacity(self.width * self.height * 7);

        for j in 0..(self.height) {
            for i in 0..(self.width) {
                output += format!("{}\n", self.frame[j][i].to_rgb_string()).as_str();
            }
        }
        // let k = self.frame[0][0].to_rgb_string();
        output
    }
}
