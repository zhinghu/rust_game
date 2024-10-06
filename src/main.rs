#![allow(non_snake_case)]

use kernel::{
    components::{camera::Camera2D, scene::Scene2D},
    render_base::Renderer,
    tools::Transform,
};
extern crate glam as glm;

mod kernel;

fn main() {
    if cfg!(debug_assertions) {
        std::env::set_var("RUST_LOG", "trace");
    } else {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();
    let mut count: i32 = 0;
    let rng = rand::thread_rng();
    let mut renderer = kernel::TerminalRenderer::new(palette::LinSrgb::new(0.0, 0.0, 0.0));
    let mut camera = Camera2D::new(glm::vec2(0.0, 0.0), 0.0, glm::vec2(5.0, 5.0));
    let scene = Scene2D::new();
    for i in 0..100 {
        scene.add();
    }
    loop {
        let canvas_size = *renderer.get_canvas().clone().size(None);
        count += 1;
        //

        renderer.render();
    }
}
