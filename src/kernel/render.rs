use std::sync::{Mutex, RwLock};

lazy_static::lazy_static! {
    static ref pixels: RwLock<Vec<FrameBuffer>> = {RwLock::new(Vec::with_capacity(
        (termsize::get().unwrap().cols * termsize::get().unwrap().rows) as usize,
    ))};
}
static INITED: Mutex<bool> = Mutex::new(false);

#[derive(Clone)]
pub struct FrameBuffer {
    pub color: glm::Vector3<f32>,
    pub deepin: f32,
}
impl FrameBuffer {
    pub fn new(color: glm::Vector3<f32>, deepin: f32) -> FrameBuffer {
        FrameBuffer { color, deepin }
    }
}

pub fn init() {
    if *INITED.lock().unwrap() {
        return;
    }
    *INITED.lock().unwrap() = true;
    pixels.write().unwrap().resize(
        (termsize::get().unwrap().cols * termsize::get().unwrap().rows) as usize,
        FrameBuffer::new(
            glm::vec3(-1.0, -1.0, -1.0),
            0.0,
        ),
    );
}

pub fn render() {
    todo!("渲染没做");
}
