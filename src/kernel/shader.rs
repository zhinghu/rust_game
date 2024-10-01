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

pub fn add_vs<F: Fn() + Send + Sync + 'static>(name_id: &str, vs: F) {
    VSHADERS.write().unwrap().insert(
        name_id.to_string(),
        Box::new(VertexShader {
            program: Box::new(vs),
        }),
    );
}

pub fn add_fs<F: Fn() + Send + Sync + 'static>(name_id: &str, fs: F) {
    FSHADERS.write().unwrap().insert(
        name_id.to_string(),
        Box::new(FragmentShader {
            program: Box::new(fs),
        }),
    );
}
