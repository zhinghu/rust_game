use std::marker::{Send, Sync};

use super::console;

#[derive(Debug, Clone, Copy)]
pub struct FData {
    pub position: glm::Vector2<f32>,
    pub rgb: glm::Vector3<f32>,
}

pub trait FShader {
    fn main(&self, data: FData) -> FData;
    fn get_name(&self) -> String;
}

pub type shaders_type = Vec<Shader>;
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

pub struct Shader {
    pub status: bool,
    pub shader: shader_type,
    pub name: String,
}
static mut shaders: Vec<Shader> = Vec::new();

pub fn add(shader: shader_type, status: bool) {
    csl_info!("add {} shader", shader.get_name());
    let name = shader.get_name();
    unsafe {
        shaders.push(Shader {
            status,
            shader: shader,
            name,
        })
    };
}

pub fn get_shaders() -> &'static shaders_type {
    unsafe { &shaders }
}
