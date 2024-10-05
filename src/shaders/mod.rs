use std::sync::Once;

lazy_static::lazy_static! {
        static ref CALLED_ONCE: Once = Once::new();
}

mod fsfile_test;use fsfile_test::fs_test;
mod vsfile_test;use vsfile_test::vs_test;

pub fn init() {CALLED_ONCE.call_once(|| {
     super::kernel::shader::add(super::kernel::shader::Shader::FragmentShader, "test", fs_test, fsfile_test::SHADER_ACTIVE);
     super::kernel::shader::add(super::kernel::shader::Shader::VertexShader, "test", vs_test, vsfile_test::SHADER_ACTIVE);
});}
