use std::sync::Once;

lazy_static::lazy_static! {
        static ref CALLED_ONCE: Once = Once::new();
}

mod vsfile_test;use vsfile_test::vs_test;
mod fsfile_test;use fsfile_test::fs_test;

pub fn init() {CALLED_ONCE.call_once(|| {
     super::kernel::shader::add_vs("test", vs_test);
     super::kernel::shader::add_fs("test", fs_test);
});}
