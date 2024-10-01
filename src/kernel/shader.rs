use std::collections::HashMap;
use std::sync::RwLock;

struct VertexShader {
    program: Box<dyn Fn() + Send + Sync + 'static>,
}
struct FragmentShader {
    program: Box<dyn Fn() + Send + Sync + 'static>,
}

lazy_static::lazy_static! {
    static ref VSHADERS: RwLock<HashMap<String, Box<VertexShader>>> = {
        RwLock::new(HashMap::new())
    };
    static ref FSHADERS: RwLock<HashMap<String, Box<FragmentShader>>> = {
        RwLock::new(HashMap::new())
    };
}

pub enum Shader {
    VertexShader,
    FragmentShader,
}

pub fn add<F: Fn() + Send + Sync + 'static>(shader_type: Shader, name_id: &str, shader: F) {
    match shader_type {
        Shader::VertexShader => {
            VSHADERS.write().unwrap().insert(
                name_id.to_string(),
                Box::new(VertexShader {
                    program: Box::new(shader),
                }),
            );
        }
        Shader::FragmentShader => {
            FSHADERS.write().unwrap().insert(
                name_id.to_string(),
                Box::new(FragmentShader {
                    program: Box::new(shader),
                }),
            );
        }
    }
}

pub fn remove(shader_type: Shader, name_id: &str) {
    match shader_type {
        Shader::VertexShader => {
            VSHADERS.write().unwrap().remove(name_id).unwrap();
        }
        Shader::FragmentShader => {
            FSHADERS.write().unwrap().remove(name_id).unwrap();
        }
    }
}
