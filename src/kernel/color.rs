use glm::*;

pub fn to_rgba(v4: Vector4<f32>) -> Vector4<f32> {
    let ones = vec4(1.0, 1.0, 1.0, 1.0);
    let rgba = vec4(127.5, 127.5, 127.5, 127.5);
    (v4 + ones) * rgba
}
