use std::{cell::RefCell, sync::Arc};

use glm::Vec3Swizzles;

use crate::kernel::{render_base::Canvas, tools};

pub trait Scene {
    fn add(&mut self, data: RefCell<Scene2DData>);
    fn put_color(&self, canvas: &mut Canvas, color: palette::LinSrgba);
}
pub trait SceneData {
    fn transform(&self) -> Vec<[glm::Vec2; 3]>;
}

pub struct Scene2DData {
    pub vpos: Vec<[glm::Vec2; 3]>,
    pub pos: glm::Vec2,
    // pub material:
    // pub texture:
    pub rotate: f32,
    pub scale: glm::Vec2,
}
impl Scene2DData {
    pub fn new() -> Self {
        Self {
            vpos: Vec::new(),
            pos: glm::vec2(0.0, 0.0),
            rotate: 0.0,
            scale: glm::vec2(1.0, 1.0),
        }
    }
}
impl SceneData for Scene2DData {
    fn transform(&self) -> Vec<[glm::Vec2; 3]> {
        let mut result: Vec<[glm::Vec2; 3]> = Vec::new();
        let M = glm::mat3a(
            glm::vec3a(1.0, 0.0, self.pos.x),
            glm::vec3a(0.0, 1.0, self.pos.y),
            glm::vec3a(0.0, 0.0, 1.0),
        ) * glm::mat3a(
            glm::vec3a(self.scale.x, 0.0, 0.0),
            glm::vec3a(0.0, self.scale.y, 0.0),
            glm::vec3a(0.0, 0.0, 1.0),
        ) * glm::mat3a(
            glm::vec3a(
                (self.rotate / 360.0).cos(),
                -(self.rotate / 360.0).sin(),
                0.0,
            ),
            glm::vec3a(
                (self.rotate / 360.0).sin(),
                (self.rotate / 360.0).cos(),
                0.0,
            ),
            glm::vec3a(0.0, 0.0, 1.0),
        );
        for v in self.vpos.iter() {
            result.push([
                (M * v[0].extend(1.0).zyx()).xy(),
                (M * v[1].extend(1.0).zyx()).xy(),
                (M * v[2].extend(1.0).zyx()).xy(),
            ]);
        }

        result
    }
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
    data: RefCell<Vec<Arc<RefCell<Scene2DData>>>>,
}

impl Scene2D {
    #[must_use]
    pub fn new() -> Self {
        Self {
            data: RefCell::new(Vec::new()),
        }
    }
}
impl Scene for Scene2D {
    fn add(&mut self, data: RefCell<Scene2DData>) {
        self.data.borrow_mut().push(Arc::new(data));
    }
    fn put_color(&self, canvas: &mut Canvas, color: palette::LinSrgba) {
        for d in self.data.borrow().clone() {
            for v in d.borrow().vpos.iter() {
                canvas.put_color(v[0], color);
                canvas.put_color(v[1], color);
                canvas.put_color(v[2], color);
            }
        }
    }
}
