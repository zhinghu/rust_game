mod kernel;
mod shaders;

fn main() {
    if cfg!(debug_assertions) {
        std::env::set_var("RUST_LOG", "trace");
    } else {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();
    shaders::init();
    let renderer = kernel::Terminal_renderer::new();
}
