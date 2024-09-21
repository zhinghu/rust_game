use std::sync::{Mutex, RwLock};

lazy_static::lazy_static! {
    static ref pixels: RwLock<Vec<glm::Vector3<f32>>> = {RwLock::new(Vec::with_capacity(
        (termsize::get().unwrap().cols * termsize::get().unwrap().rows) as usize,
    ))};
}
static INITED: Mutex<bool> = Mutex::new(false);

pub fn init() {
    *INITED.lock().unwrap() = true;
    pixels.write().unwrap().resize(
        (termsize::get().unwrap().cols * termsize::get().unwrap().rows) as usize,
        glm::vec3(-1.0, -1.0, -1.0),
    );
}
