use ext::*;
use glm::*;

#[derive(Debug)]
pub struct Position {
    pub xyz: Vector3<f32>,
    pub radians: f32,
    pub rotate_p: Vector3<f32>,
    pub translation: Vector3<f32>,
    pub translation_active: bool,
    pub scale: Vector3<f32>,
}

impl Position {
    fn process(&self) -> Vec3 {
        let mat4_1 = mat4(
            1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        );
        let rotate_mat = rotate(&mat4_1, radians(self.radians), self.rotate_p);
        let translation_mat = translate(&mat4_1, self.translation);
        let scale_mat = scale(&mat4_1, self.scale);

        (rotate_mat
            * scale_mat
            * translation_mat
            * self
                .xyz
                .extend(if self.translation_active { 1.0 } else { 0.0 }))
        .truncate(3)
    }
}
