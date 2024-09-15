use std::marker::{Send, Sync};

use super::console;

#[derive(Debug)]
pub struct FData {
    pub x: usize,
    pub y: usize,
    pub rgb: glm::Vector3<f32>,
}

pub trait FShader {
    fn main(&self, data: FData) -> FData;
    fn get_name(&self) -> String;
}

pub type shaders_type = Vec<Box<dyn FShader + Send + Sync>>;
pub type shader_type = Box<dyn FShader + 'static + Send + Sync>;
pub struct empty_shader;
impl FShader for empty_shader {
    fn main(&self, data: FData) -> FData {
        data
    }
    fn get_name(&self) -> String {
        "empty_shader".to_string()
    }
}

static mut shaders: Vec<Box<dyn FShader + Send + Sync>> = Vec::new();

pub fn add(shader: shader_type) {
    csl_info!("add {} shader", shader.get_name());
    unsafe { shaders.push(shader) };
}

pub fn get_shaders() -> &'static shaders_type {
    unsafe { &shaders }
}
