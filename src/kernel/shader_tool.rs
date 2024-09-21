pub use super::shader::*;
pub use glm::ext::*;
pub use glm::*;

pub fn delta_to_pixels(A: Vector2<f32>, B: Vector2<f32>, C: Vector2<f32>, P: Vector2<f32>) -> bool {
    let v0 = C - A;
    let v1 = B - A;
    let v2 = P - A;
    let dot00 = dot(v0, v0);
    let dot01 = dot(v0, v1);
    let dot02 = dot(v0, v2);
    let dot11 = dot(v1, v1);
    let dot12 = dot(v1, v2);
    let inver_deno = 1.0 / (dot00 * dot11 - dot01 * dot01);
    let u = (dot11 * dot02 - dot01 * dot12) * inver_deno;
    let v = (dot00 * dot12 - dot01 * dot02) * inver_deno;

    u >= 0.0 && v >= 0.0 && u + v <= 1.0
}
