use std::ops::{Add, Mul, Sub};

// Color struct holds fraction of value of RGB values not RGB values
// ex: RGB(255,255,255) <==> Color {red: 1.0, green:1.0, blue: 1.0}
// ex: RGB(100,100,100) <==> Color {red: 100/255, green:100/255, blue: 100/255}
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

    pub(crate) fn black() -> Self {
        Self::new(0.0, 0.0, 0.0)
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
        if self.red == other.red && self.green == other.green && self.blue == other.blue {
            true
        } else {
            false
        }
    }
}