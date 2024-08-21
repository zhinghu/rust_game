use glm::*;

pub fn to_rgba2(v2: Vector2<f32>) -> Vector2<f32> {
    clamp(v2, vec2(-1.0, -1.0), vec2(1.0, 1.0));
    let ones = vec2(1.0, 1.0);
    let rgba = vec2(127.5, 127.5);
    round((v2 + ones) * rgba)
}
pub fn to_rgba3(v3: Vector3<f32>) -> Vector3<f32> {
    clamp(v3, vec3(-1.0, -1.0, -1.0), vec3(1.0, 1.0, 1.0));
    let ones = vec3(1.0, 1.0, 1.0);
    let rgba = vec3(127.5, 127.5, 127.5);
    round((v3 + ones) * rgba)
}
pub fn to_rgba4(v4: Vector4<f32>) -> Vector4<f32> {
    clamp(v4, vec4(-1.0, -1.0, -1.0, -1.0), vec4(1.0, 1.0, 1.0, 1.0));
    let ones = vec4(1.0, 1.0, 1.0, 1.0);
    let rgba = vec4(127.5, 127.5, 127.5, 127.5);
    round((v4 + ones) * rgba)
}
