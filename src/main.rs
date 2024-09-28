mod kernel;

fn main() {
    if cfg!(debug_assertions) {
        std::env::set_var("RUST_LOG", "trace");
    } else {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();
    kernel::render_base::init();
}
