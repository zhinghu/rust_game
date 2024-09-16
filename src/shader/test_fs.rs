use super::super::kernel::shader::*;

pub struct test_fs;

impl FShader for test_fs {
    fn main(&self, mut data: FData) -> FData {
        data.rgb.x = data.position.x * 2.0 - 1.0;
        data.rgb.y = data.position.y * 2.0 - 1.0;

        data
    }

    fn get_name(&self) -> String {
        "test_fs".to_string()
    }
}
