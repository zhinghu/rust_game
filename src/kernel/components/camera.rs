use super::super::tools;

#[derive(Debug)]
pub struct Camera2D {
    pub pos: glm::Vec2,
    pub rotate: f32,
    pub scale: glm::Vec2,
}

impl Camera2D {
    pub fn new(pos: glm::Vec2, rotate: f32, scale: glm::Vec2) -> Self {
        Self { pos, rotate, scale }
    }
}
impl tools::Transform<glm::Mat3A> for Camera2D {
    fn process(&self) -> glm::Mat3A {
        let S = glm::mat3a(
            glm::vec3a(self.scale.x, 0.0, 0.0),
            glm::vec3a(0.0, self.scale.y, 0.0),
            glm::vec3a(0.0, 0.0, 1.0),
        );
        let R = glm::mat3a(
            glm::vec3a(
                (self.rotate / 180.0).cos(),
                -(self.rotate / 180.0).sin(),
                0.0,
            ),
            glm::vec3a(
                (self.rotate / 180.0).sin(),
                (self.rotate / 180.0).cos(),
                0.0,
            ),
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
