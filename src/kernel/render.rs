use crate::kernel::color::to_rgba3;

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
        let mut result: String = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let color = self.getPixel(x, y);
                let color = to_rgba3(color);
                result = format!(
                    "{result}\x1b[48;2;{};{};{}m\u{0020}",
                    color.x as u8, color.y as u8, color.z as u8
                );
            }
        }

        format!("{result}\x1b[0m")
    }

    pub fn getWidth(&self) -> usize {
        self.width
    }
    pub fn getHeight(&self) -> usize {
        self.height
    }
    pub fn getPixel(&self, x: usize, y: usize) -> glm::Vector3<f32> {
        self.pixels[y * self.width + x]
    }
    pub fn setPixel(&mut self, x: usize, y: usize, color: glm::Vector3<f32>) {
        self.pixels[y * self.width + x] = color;
    }
}
