use std::time::Instant;

use super::color::to_rgba3;
use super::{console, shader::*};
use rayon::prelude::*;

pub struct Render {
    width: usize,
    height: usize,
    pixels: Vec<glm::Vector3<f32>>,
    shaders: &'static shaders_type,
}

impl Render {
    pub fn new(width: usize, height: usize) -> Render {
        assert!(width != 0);
        assert!(height != 0);
        console::info("creating buffer vector".to_string());
        let mut pixels: Vec<glm::Vector3<f32>> = vec![];
        pixels.resize(width * height, glm::vec3(-1., -1., -1.));
        Render {
            width,
            height,
            pixels,
            shaders: get_shaders(),
        }
    }
    fn clamp(v3: glm::Vector3<f32>) {
        glm::clamp(v3, glm::vec3(-1.0, -1.0, -1.0), glm::vec3(1.0, 1.0, 1.0));
    }

    pub fn render(&mut self) -> String {
        let mut result: String = String::from("\x1b[0;0H");
        // apply shaders
        for s in self.shaders.iter() {
            self.pixels
                .par_iter_mut()
                .enumerate()
                .for_each(|(i, pixel)| {
                    *pixel = s
                        .main(FData {
                            x: i % self.width,
                            y: (i as f32 / self.width as f32) as usize,
                            rgb: *pixel,
                        })
                        .rgb;
                });
        }

        for y in 0..self.height {
            for x in 0..self.width {
                let color = to_rgba3(*self.getPixel(x, y));
                result = format!(
                    "{result}\x1b[48;2;{};{};{}m\u{0020}",
                    color.x as u8, color.y as u8, color.z as u8
                );
            }
        }

        format!("{}\x1b[0m", result)
    }

    pub fn getWidth(&self) -> usize {
        self.width
    }
    pub fn getHeight(&self) -> usize {
        self.height
    }
    pub fn getPixel(&self, x: usize, y: usize) -> &glm::Vector3<f32> {
        self.pixels.get(y * self.width + x).unwrap()
    }
    pub fn setPixel(&mut self, x: usize, y: usize, color: &mut glm::Vector3<f32>) {
        if let Some(pixel) = self.pixels.get_mut(y * self.width + x) {
            *pixel = *color
        };
    }
}
