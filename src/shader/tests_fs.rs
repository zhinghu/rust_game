use super::super::kernel::shader::*;

pub struct tests_fs;

impl FShader for tests_fs {
    fn main(&self, mut data: FData) -> FData {
        if data.position.x >= 0.0 && data.position.y <= 0.5 {
            data.rgb.x *= -1.0;
        }

        data
    }

    fn get_name(&self) -> String {
        "tests_fs".to_string()
    }
}
