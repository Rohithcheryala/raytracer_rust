use crate::color::{Color, ToRGB};
use rayon::iter::IntoParallelIterator;
use std::{convert::AsRef, fs, path::Path};

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

    pub fn iter(&self) -> std::slice::Iter<Vec<Color>> {
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
        // FIXME: change with_capacity value
        let mut output = String::with_capacity(self.width * self.height * 7);

        for j in 0..(self.height) {
            for i in 0..(self.width) {
                output += format!("{}\n", self.frame[j][i].to_rgb_string()).as_str();
            }
        }
        output
    }
}

// TODO: Analyse this situation
// How to render a canvas in a multi-treading way.
// Implementation 1: use mutex to share between threads
// Goods:
// 1. Easy to implement.
// 2. Convienient here cause the MutexGaurd is held for less duration (that is how it should be)
// and only once per iteration and no read operations are required here.
// Bads:
// 1. What if 4 mutating access are required per every iteration.
// Performance may be poor (need reasoning?)

// Implementation 2: implementing rayon threads for struct Canvas, its in a way redirecting it to underlying struct Vec cause Canvas is just a wrapper around struct Vec with some other fields,
// Goods:
// 1. Canvas itself is shared between threads and without any conflicts,
// can simutaneously write 2 pixels information at a time.(for now the writing part time is negligible)
// Bads
// 1. Cant access nieghbour pixel colors if needed in future.

impl IntoParallelIterator for Canvas {
    type Item = Vec<Color>;
    type Iter = rayon::vec::IntoIter<Self::Item>;

    fn into_par_iter(self) -> Self::Iter {
        self.frame.into_par_iter()
    }
}

impl<'data> IntoParallelIterator for &'data Canvas {
    type Item = &'data Vec<Color>;
    type Iter = rayon::slice::Iter<'data, Vec<Color>>;

    fn into_par_iter(self) -> Self::Iter {
        <&[Vec<Color>]>::into_par_iter(self.frame.as_slice())
    }
}

impl<'data> IntoParallelIterator for &'data mut Canvas {
    type Item = &'data mut Vec<Color>;
    type Iter = rayon::slice::IterMut<'data, Vec<Color>>;

    fn into_par_iter(self) -> Self::Iter {
        <&mut [Vec<Color>]>::into_par_iter(self.frame.as_mut_slice())
    }
}
