use crate::consts::EPSILON;
use std::ops::{Add, Mul, Sub};

pub trait RGB {
    fn red(&self) -> f64;
    fn green(&self) -> f64;
    fn blue(&self) -> f64;
}

pub trait ToRGB: RGB {
    fn to_rgb_string(&self) -> String {
        format!(
            "{} {} {}",
            (self.red() * 255_f64) as i64,
            (self.green() * 255_f64) as i64,
            (self.blue() * 255_f64) as i64,
        )
    }
}
/*
/ Color struct holds fraction of value of RGB values not RGB values
/ ex: RGB(255,255,255) <==> Color {red: 1.0, green:1.0, blue: 1.0}
/ ex: RGB(100,100,100) <==> Color {red: 100/255, green:100/255, blue: 100/255}
 */
#[derive(Clone, Copy, Debug)]
pub struct Color {
    red: f64,
    green: f64,
    blue: f64,
}

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Self { red, green, blue }
    }

    #[inline]
    pub fn white() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }

    #[inline]
    pub fn black() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

impl RGB for Color {
    fn red(&self) -> f64 {
        self.red
    }
    fn green(&self) -> f64 {
        self.green
    }

    fn blue(&self) -> f64 {
        self.blue
    }
}
impl ToRGB for Color {}

impl Add for Color {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self::Output {
        self.red += rhs.red;
        self.green += rhs.green;
        self.blue += rhs.blue;
        self
    }
}

impl Sub for Color {
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self::Output {
        self.red -= rhs.red;
        self.green -= rhs.green;
        self.blue -= rhs.blue;
        self
    }
}

impl Mul for Color {
    type Output = Self;
    fn mul(mut self, rhs: Self) -> Self::Output {
        self.red *= rhs.red;
        self.green *= rhs.green;
        self.blue *= rhs.blue;
        self
    }
}

impl<T> Mul<T> for Color
where
    T: Into<f64> + Copy,
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self {
            red: self.red * rhs.into(),
            green: self.green * rhs.into(),
            blue: self.blue * rhs.into(),
        }
    }
}

/// Default color is white(1, 1, 1)
impl Default for Color {
    fn default() -> Self {
        Self {
            red: 1.0,
            green: 1.0,
            blue: 1.0,
        }
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        for (a, b) in [
            (self.red, other.red),
            (self.green, other.green),
            (self.blue, other.blue),
        ] {
            if (a - b).abs() > EPSILON {
                return false;
            }
        }
        true
    }
}
