use std::borrow::Borrow;

use super::color::to_rgba3;
use super::shader::*;

pub struct Render {
    width: usize,
    height: usize,
    pixels: Vec<glm::Vector3<f32>>,
}

impl Render {
    pub fn new(width: usize, height: usize) -> Render {
        assert!(width > 0);
        assert!(height > 0);
        let mut pixels: Vec<glm::Vector3<f32>> = vec![];
        pixels.resize(width * height, glm::vec3(-1., -1., -1.));
        Render {
            width,
            height,
            pixels,
        }
    }
    fn clamp(v3: glm::Vector3<f32>) {
        glm::clamp(v3, glm::vec3(-1.0, -1.0, -1.0), glm::vec3(1.0, 1.0, 1.0));
    }

    pub fn render(&self) -> String {
        let mut result: String = String::from("");
        for y in 0..self.height {
            for x in 0..self.width {
                let color = self.getPixel(x, y);
                let color = to_rgba3(*color);
                result = format!(
                    "{result}\x1b[48;2;{};{};{}m\u{0020}",
                    color.x as u8, color.y as u8, color.z as u8
                );
            }
        }

        format!("{result}\x1b[0m")
    }

    pub fn use_shader(&mut self, shaders: &'static shaders_type) {
        for si in 0..shaders.len() {
            for i in 0..self.pixels.len() {
                if let Some(pixel) = self.pixels.get_mut(i) {
                    *pixel = shaders
                        .get(si)
                        .unwrap()
                        .main(FData {
                            x: i % self.width,
                            y: (i as f32 / self.width as f32) as usize,
                            rgb: *pixel,
                        })
                        .rgb;
                }
            }
        }
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
