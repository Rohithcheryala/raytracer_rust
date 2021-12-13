use std::ops::{Add, Mul, Sub};

#[derive(Clone, Copy)]
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
    pub(crate) fn to_value_string(&self) -> String {
        format!(
            "{} {} {}",
            (self.red * 255_f64) as i64,
            (self.green * 255_f64) as i64,
            (self.blue * 255_f64) as i64,
        )
    }
}

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

impl Default for Color {
    fn default() -> Self {
        Self {
            red: 255.0,
            green: 255.0,
            blue: 255.0,
        }
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        if self.red == other.red && self.green == other.green && self.blue == other.blue {
            true
        } else {
            false
        }
    }
}
