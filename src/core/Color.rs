pub struct Color {
    pub vec4: [f32; 4],
}

impl Color {
    pub fn new(vec4: &[f32; 4]) -> Color {
        Color { vec4: *vec4 }
    }
    // Get RGBA values(0-255)
    pub fn get(&self) -> [u8; 4] {
        let vec4: [f32; 4] = self.vec4;
        let mut result: [u8; 4] = [0, 0, 0, 0];
        for n in 0..vec4.len() {
            result[n] = ((vec4[n].clamp(-1.0, 1.0) + 1.0) * 127.5) as u8;
        }

        result
    }
}
