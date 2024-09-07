#[derive(Debug)]
pub struct FData {
    pub x: usize,
    pub y: usize,
    pub rgb: glm::Vector3<f32>,
}

pub trait FShader {
    fn main(&self, data: FData) -> FData;
}

pub type shaders_type = Vec<Box<dyn FShader>>;
pub type shader_type = Box<dyn FShader + 'static>;
pub struct empty_shader;
impl FShader for empty_shader {
    fn main(&self, data: FData) -> FData {
        data
    }
}

static mut shaders: Vec<Box<dyn FShader>> = Vec::new();

pub fn add(shader: shader_type) {
    unsafe { shaders.push(shader) };
}

pub fn get_shaders() -> &'static shaders_type {
    unsafe { &shaders }
}
