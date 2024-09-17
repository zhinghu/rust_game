use std::f64;

use super::super::kernel::shader::*;

pub struct test_fs;

impl FShader for test_fs {
    fn main(&self, mut data: FData) -> FData {
        let time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();

        data.rgb.x = f64::sin(time) as f32;
        data.rgb.y = f64::cos(time) as f32;
        data.rgb.z = f64::sin(f64::cos(time)) as f32;

        data
    }

    fn get_name(&self) -> String {
        "test_fs".to_string()
    }
}
