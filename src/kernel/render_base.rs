use std::sync::RwLock;

use log::info;

lazy_static::lazy_static! {
    pub static ref pixels: RwLock<Vec<FrameBuffer>> = RwLock::new(Vec::new());
}

#[derive(Clone)]
pub struct FrameBuffer {
    pub color: glm::Vector3<f32>,
    pub deepin: f32,
}
impl FrameBuffer {}

pub fn init(width: usize, height: usize) {
    info!("initing render_base module");
    pixels.write().unwrap().resize(
        width * height,
        FrameBuffer {
            color: glm::vec3(-1.0, -1.0, -1.0),
            deepin: 0.0,
        },
    );
}

pub trait Renderer {
    fn render(&self);
}
