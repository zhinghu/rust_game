pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    /**
     * color_vec: -1 ~ 1(0 ~ 255)
     */
    pub fn new(color_vec: &[f32; 4]) -> Color {
        let color = Color {
            r: ((color_vec[0] + 1.0) * 127.5) as u8,
            g: ((color_vec[1] + 1.0) * 127.5) as u8,
            b: ((color_vec[2] + 1.0) * 127.5) as u8,
            a: ((color_vec[3] + 1.0) * 127.5) as u8,
        };
        color
    }

    pub fn get(&self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }
}
