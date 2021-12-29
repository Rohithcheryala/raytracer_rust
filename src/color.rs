use crate::EPSILON;
use std::ops::{Add, Mul, Sub};

pub(crate) trait RGB {
    fn red(&self) -> f64;
    fn green(&self) -> f64;
    fn blue(&self) -> f64;
}

pub(crate) trait ToRGB {
    fn to_rgb_string(&self) -> String
    where
        Self: RGB,
    {
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
pub(crate) struct Color {
    red: f64,
    green: f64,
    blue: f64,
}

impl Color {
    pub(crate) fn new(red: f64, green: f64, blue: f64) -> Self {
        Self { red, green, blue }
    }

    #[inline]
    pub(crate) fn white() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }

    #[inline]
    pub(crate) fn black() -> Self {
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
        self.red = self.red + rhs.red;
        self.green = self.green + rhs.green;
        self.blue = self.blue + rhs.blue;
        self
    }
}

impl Sub for Color {
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self::Output {
        self.red = self.red - rhs.red;
        self.green = self.green - rhs.green;
        self.blue = self.blue - rhs.blue;
        self
    }
}

impl Mul for Color {
    type Output = Self;
    fn mul(mut self, rhs: Self) -> Self::Output {
        self.red = self.red * rhs.red;
        self.green = self.green * rhs.green;
        self.blue = self.blue * rhs.blue;
        self
    }
}

impl<T> Mul<T> for Color
where
    f64: From<T>,
    T: Copy,
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self {
            red: self.red * f64::from(rhs),
            green: self.green * f64::from(rhs),
            blue: self.blue * f64::from(rhs),
        }
    }
}

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
