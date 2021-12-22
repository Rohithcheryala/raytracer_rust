use crate::color::Color;

pub(crate) struct Canvas {
    width: usize,
    height: usize,
    frame: Vec<Vec<Color>>,
}

// TODO: replace all(max) Vec::new() with Vec::with_capacity()
// can be done it many places as we already nrows,ncols
// ex: in new() ~~~let mut mat = Vec::new()~~~ => to Vec::with_capacity(height)
impl Canvas {
    pub(crate) fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            frame: {
                let mut mat = Vec::new();
                for _j in 0..height {
                    let mut m = Vec::new();
                    for _i in 0..width {
                        m.push(Color::new(0.0, 0.0, 0.0))
                    }
                    mat.push(m);
                }
                mat
            },
        }
    }

    pub(crate) fn set_color_at_pixel(&mut self, x: f64, y: f64, color: Color) {
        self.frame[y as usize][x as usize] = color;
    }

    pub(crate) fn set_color_at_pixels(&mut self, x: f64, y: f64, color: Color) {
        for i in (x - 1f64) as i64..(x + 1f64) as i64 {
            for j in (y - 1f64) as i64..(y + 1f64) as i64 {
                self.set_color_at_pixel(i as f64, j as f64, color)
            }
        }
    }

    pub(crate) fn save_as_ppm(&self, save_to: String) -> Result<(), String> {
        let mut output = String::new();
        output = output + format!("P3\n{} {}\n255\n", self.width, self.height).as_str();

        for j in 0..(self.height as i64) {
            for i in 0..(self.width as i64) {
                output = output
                    + format!("{}\n", self.frame[j as usize][i as usize].to_value_string())
                        .as_str();
            }
        }

        std::fs::write(save_to, output).unwrap();
        Ok(())
    }
}
