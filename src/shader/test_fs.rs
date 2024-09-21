use super::super::kernel::shader_tool::*;

pub struct test_fs;

impl FShader for test_fs {
    fn main(&self, mut data: FData) -> FData {
        if delta_to_pixels(
            vec2(-0.3, -0.3),
            vec2(-0.3, 0.3),
            vec2(0.3, -0.3),
            data.position,
        ) || delta_to_pixels(
            vec2(0.3, -0.3),
            vec2(0.3, 0.3),
            vec2(-0.3, 0.3),
            data.position,
        ) {
            data.rgb.x = data.position.x;
            data.rgb.y = data.position.y;
        }

        data
    }

    fn get_name(&self) -> String {
        "test_fs".to_string()
    }
}
