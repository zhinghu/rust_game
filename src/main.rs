use kernel::render_base::Renderer;

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
    let mut renderer = kernel::Terminal_renderer::new();
    let rd = renderer.get_mut_canvas();

    rd.fill_color(glm::vec4(1.0, -1.0, -1.0, 1.0));
    rd.fill_color(glm::vec4(-1.0, 1.0, -1.0, -0.9));

    renderer.render();
}
