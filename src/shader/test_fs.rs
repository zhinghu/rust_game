use super::super::kernel::shader::*;

pub struct test_fs;

impl FShader for test_fs {
    fn main(&self, mut data: FData) -> FData {
        let _ = termsize::get().into_iter().map(|size| {
            data.rgb.x = data.x as f32 / size.cols as f32;
            data.rgb.y = data.y as f32 / size.rows as f32;
        });

        data
    }
}
