use super::color::to_rgba3;
use super::{console, shader::*};
use rayon::prelude::*;

pub struct Render {
    width: usize,
    height: usize,
    pixels: Vec<FData>,
    shaders: &'static shaders_type,
}

impl Render {
    pub fn new(width: usize, height: usize) -> Render {
        assert!(width != 0);
        assert!(height != 0);
        csl_info!("creating buffer vector");
        csl_debug!("width * height: {}", width * height);
        let mut pixels: Vec<FData> = Vec::with_capacity(width * height);
        csl_debug!("vector len: {}", pixels.len());
        pixels.resize(
            width * height,
            FData {
                position: glm::vec2(0.0, 0.0),
                rgb: glm::vec3(-1.0, -1.0, -1.0),
            },
        );
        pixels.par_iter_mut().enumerate().for_each(|(i, pixel)| {
            *pixel = FData {
                rgb: glm::vec3(-1.0, -1.0, -1.0),
                position: glm::vec2(
                    (i % width) as f32 / width as f32,
                    (i as f32 / width as f32) / height as f32,
                ),
            };
        });
        csl_debug!("inited vector len: {}", pixels.len());
        Render {
            width,
            height,
            pixels,
            shaders: get_shaders(),
        }
    }

    pub fn render(&mut self) -> String {
        let mut result: String = String::from("\x1b[0;0H");

        self.use_shader();

        for y in 0..self.height {
            for x in 0..self.width {
                let color = to_rgba3(self.get_pixel(x, y).rgb);
                result = format!(
                    "{result}\x1b[48;2;{};{};{}m\u{0020}",
                    color.x as u8, color.y as u8, color.z as u8
                );
            }
        }

        format!("{}\x1b[0m", result)
    }

    pub fn use_shader(&mut self) {
        // apply shaders
        for s in self.shaders.iter() {
            self.pixels.par_iter_mut().for_each(|pixel| {
                *pixel = s.main(*pixel);
            });
        }
    }
    pub fn get_width(&self) -> usize {
        self.width
    }
    pub fn get_height(&self) -> usize {
        self.height
    }
    pub fn get_pixel(&self, x: usize, y: usize) -> &FData {
        self.pixels.get(y * self.width + x).unwrap()
    }
    pub fn set_pixel(&mut self, x: usize, y: usize, color: &FData) {
        if let Some(pixel) = self.pixels.get_mut(y * self.width + x) {
            *pixel = *color
        };
    }
}
