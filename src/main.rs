mod kernel;
mod shader;
use std::borrow::Borrow;

use kernel::console;

fn main() {
    println!(
        "Version: {}-{}
Author: Github mychinesepyl",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );
    println!("");
    let mut term_w: usize = 1;
    let mut term_h: usize = 1;
    termsize::get().map(|size| {
        term_w = size.cols as usize;
        term_h = size.rows as usize;
    });
    let mut color = kernel::Render::new(term_w, term_h);

    /* awa
    for y in 0..color.getHeight() {
        for x in 0..color.getWidth() {
            color.setPixel(
                x,
                y,
                &mut glm::vec3(
                    ((x as f32 / color.getWidth() as f32) * 2.0 - 1.0) as f32,
                    ((y as f32 / color.getHeight() as f32) * 2.0 - 1.0) as f32,
                    -1.0,
                ),
            );
        }
    }

    // console::debug(format!("{}", color.render())); */
    kernel::shader::add(Box::new(shader::test_fs::test_fs));
    color.use_shader(kernel::shader::get_shaders());
    println!("{}", color.render());
}
