use std::sync::RwLock;

use crate::kernel::tools;

pub trait Scene {}
pub trait SceneData {}

pub struct Scene2DData {
    pub vpos: Vec<[glm::Vec2; 3]>,
    pub pos: glm::Vec2,
    // pub material:
    // pub texture:
    pub rotate: f32,
    pub scale: glm::Vec2,
}
impl tools::Transform<glm::Mat3A> for Scene2DData {
    #[must_use]
    fn process(&self) -> glm::Mat3A {
        let S = glm::mat3a(
            glm::vec3a(self.scale.x, 0.0, 0.0),
            glm::vec3a(0.0, self.scale.y, 0.0),
            glm::vec3a(0.0, 0.0, 1.0),
        );
        let R = glm::mat3a(
            glm::vec3a(self.rotate.cos(), -self.rotate.sin(), 0.0),
            glm::vec3a(self.rotate.sin(), self.rotate.cos(), 0.0),
            glm::vec3a(0.0, 0.0, 1.0),
        );
        let T = glm::mat3a(
            glm::vec3a(1.0, 0.0, self.pos.x),
            glm::vec3a(0.0, 1.0, self.pos.y),
            glm::vec3a(0.0, 0.0, 1.0),
        );
        T * S * R
    }
}
pub struct Scene2D {
    data: RwLock<Vec<Scene2DData>>,
}

impl Scene2D {
    #[must_use]
    pub fn new() -> Self {
        Self {
            data: RwLock::new(Vec::new()),
        }
    }
}
