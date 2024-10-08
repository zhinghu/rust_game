use std::io::{self, Write};

use super::color::to_rgba3;
use super::{console, shader::*};
use rayon::prelude::*;

pub struct Render {
    width: usize,
    height: usize,
    pixels: Vec<FData>,
    old_pixels: Vec<FData>,
    shaders: &'static shaders_type,
}

impl Render {
    pub fn new() -> Render {
        let (width, height) = (
            termsize::get().unwrap().cols as usize,
            termsize::get().unwrap().rows as usize,
        );
        csl_info!("creating buffer vector");
        csl_debug!("width * height: {}", width * height);
        let mut pixels: Vec<FData> = Vec::with_capacity(width * height);
        csl_debug!("vector len: {}", pixels.len());
        pixels.resize(
            width * height,
            FData {
                position: glm::vec2(-1.0, -1.0),
                rgb: glm::vec3(-1.0, -1.0, -1.0),
            },
        );
        pixels.par_iter_mut().enumerate().for_each(|(i, pixel)| {
            *pixel = FData {
                rgb: glm::vec3(-1.0, -1.0, -1.0),
                position: glm::vec2(
                    (i % width) as f32 / width as f32 * 2.0 - 1.0,
                    (i as f32 / width as f32) / height as f32 * 2.0 - 1.0,
                ),
            };
        });
        let old_pixels = pixels.clone();
        csl_debug!("inited vector len: {}", pixels.len());
        Render {
            width,
            height,
            pixels,
            old_pixels,
            shaders: get_shaders(),
        }
    }

    pub fn render(&mut self) -> io::Result<()> {
        let nsize = termsize::get().unwrap();
        if self.pixels.len() != nsize.rows as usize * nsize.cols as usize {
            self.pixels.resize(
                nsize.rows as usize * nsize.cols as usize,
                FData {
                    position: glm::vec2(-1.0, -1.0),
                    rgb: glm::vec3(-1.0, -1.0, -1.0),
                },
            );
            self.width = nsize.cols as usize;
            self.height = nsize.rows as usize;
            self.pixels
                .par_iter_mut()
                .enumerate()
                .for_each(|(i, pixel)| {
                    *pixel = FData {
                        rgb: glm::vec3(-1.0, -1.0, -1.0),
                        position: glm::vec2(
                            (i % self.width) as f32 / self.width as f32 * 2.0 - 1.0,
                            (i as f32 / self.width as f32) / self.height as f32 * 2.0 - 1.0,
                        ),
                    };
                });
        }

        let mut result = io::BufWriter::new(io::stdout().lock());
        result.write("\x1b[0;0H".as_bytes())?;
        self.old_pixels = self.pixels.clone();

        // apply shaders
        self.shaders.iter().filter(|&s| s.status).for_each(|s| {
            self.pixels.par_iter_mut().for_each(|pixel| {
                *pixel = s.shader.main(*pixel);
            });
        });

        self.pixels.iter().for_each(|pixel| {
            let color = to_rgba3(pixel.rgb);
            result
                .write_all(
                    format!(
                        "\x1b[48;2;{};{};{}m\u{0020}",
                        color.x as u8, color.y as u8, color.z as u8
                    )
                    .as_bytes(),
                )
                .unwrap();
        });

        result.write_all("\x1b[0m".as_bytes())?;
        Ok(())
    }

    pub fn get_width(&self) -> usize {
        self.width
    }
    pub fn get_height(&self) -> usize {
        self.height
    }
    pub fn get_pixels(&self) -> &Vec<FData> {
        &self.pixels
    }
}
