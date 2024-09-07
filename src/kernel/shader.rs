pub struct FData {
    pub x: usize,
    pub y: usize,
    pub rgb: glm::Vector3<f32>,
}

pub trait FShader {
    fn main(&self, data: FData) -> FData;
}

static mut shaders: Vec<Box<dyn FShader>> = Vec::new();

pub fn add(shader: Box<dyn FShader + 'static>) {
    unsafe { shaders.push(shader) };
}
