use glm::*;

pub fn color_to_term(v4: Vector4<f32>) -> String {
    format!(
        "\x1b[48;2;{:?};{:?};{:?}m\u{0020}\x1b[0m",
        v4.x as u8, v4.y as u8, v4.z as u8
    )
}
