use std::collections::HashMap;
use std::sync::{RwLock, RwLockReadGuard};

use log::debug;

pub struct VertexShader {
    program: Box<dyn Fn() + Send + Sync + 'static>,
    shader_active: bool,
}
pub struct FragmentShader {
    program: Box<dyn Fn() + Send + Sync + 'static>,
    shader_active: bool,
}

impl std::fmt::Debug for VertexShader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VertexShader")
            .field("program", &"Fn()")
            .field("shader_active", &self.shader_active.to_string().as_str())
            .finish()
    }
}
impl std::fmt::Debug for FragmentShader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FragmentShader")
            .field("program", &"Fn()")
            .field("shader_active", &self.shader_active.to_string().as_str())
            .finish()
    }
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

/// 向着色器列表里添加着色器
pub fn add<F: Fn() + Send + Sync + 'static>(
    shader_type: Shader,
    name_id: &str,
    shader: F,
    shader_active: bool,
) {
    match shader_type {
        Shader::VertexShader => {
            VSHADERS.write().unwrap().insert(
                name_id.to_string(),
                Box::new(VertexShader {
                    program: Box::new(shader),
                    shader_active,
                }),
            );
            debug!(
                "add VertexShader with {}, active: {}",
                name_id, shader_active
            );
        }
        Shader::FragmentShader => {
            FSHADERS.write().unwrap().insert(
                name_id.to_string(),
                Box::new(FragmentShader {
                    program: Box::new(shader),
                    shader_active,
                }),
            );
            debug!(
                "add FragmentShader with {}, active: {}",
                name_id, shader_active
            );
        }
    }
}

/// 移除着色器列表里的着色器
pub fn remove(shader_type: Shader, name_id: &str) {
    match shader_type {
        Shader::VertexShader => {
            VSHADERS.write().unwrap().remove(name_id).unwrap();
            debug!("remove VertexShader with {}", name_id);
        }
        Shader::FragmentShader => {
            FSHADERS.write().unwrap().remove(name_id).unwrap();
            debug!("remove FragmentShader with {}", name_id);
        }
    }
}

pub fn set_active(shader_type: Shader, name_id: &str, shader_active: bool) {
    match shader_type {
        Shader::VertexShader => {
            VSHADERS
                .write()
                .unwrap()
                .get_mut(name_id)
                .unwrap()
                .shader_active = shader_active;
        }
        Shader::FragmentShader => {
            FSHADERS
                .write()
                .unwrap()
                .get_mut(name_id)
                .unwrap()
                .shader_active = shader_active;
        }
    }
}

/// 获取着色器列表为元组
pub fn get<'a>() -> (
    RwLockReadGuard<'a, HashMap<String, Box<VertexShader>>>,
    RwLockReadGuard<'a, HashMap<String, Box<FragmentShader>>>,
) {
    (VSHADERS.read().unwrap(), FSHADERS.read().unwrap())
}
