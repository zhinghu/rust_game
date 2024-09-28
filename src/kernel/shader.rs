use std::collections::HashMap;
use std::sync::RwLock;

pub struct VertexShader {
    program: Box<dyn Fn() + Send + Sync + 'static>,
}
pub struct FragmentShader {
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
