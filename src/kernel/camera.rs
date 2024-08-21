use super::{Ray, Render};

pub struct Camera {
    data: Render,
    pos: glm::Vector3<f32>,
    camera_from_clip: glm::Matrix4<f32>,
    world_from_camera: glm::Matrix4<f32>,
}

impl Camera {
    pub fn new(
        &mut self,
        data: Render,
        pos: glm::Vector3<f32>,
        viewpoint: glm::Vector3<f32>,
        fovy: f32,
    ) -> Camera {
        let camera_from_clip = glm::inverse(&glm::ext::perspective(
            glm::radians(fovy),
            self.data.getWidth() as f32 / self.data.getHeight() as f32,
            1.0,
            2.0,
        ));

        Camera {
            data,
            pos,
            camera_from_clip,
        }
    }

    pub fn generateRay(pixel_coord: glm::Vector2<i32>, offset: Option<glm::Vector2<f32>>) -> Ray {
        let offset = offset.unwrap_or(glm::vec2(0.5, 0.5));
        Ray {}
    }

    pub fn getData(&self) -> Render {
        //
    }
}
