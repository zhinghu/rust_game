use super::super::console;
use super::super::kernel::shader::*;

pub struct test_fs;

impl FShader for test_fs {
    fn main(&self, mut data: FData) -> FData {
        let size = termsize::get().unwrap();
        data.rgb.x = data.x as f32 / size.cols as f32 * 2.0 - 1.0;
        data.rgb.y = data.y as f32 / size.rows as f32 * 2.0 - 1.0;

        data
    }

    fn get_name(&self) -> String {
        "test_fs".to_string()
    }
}
