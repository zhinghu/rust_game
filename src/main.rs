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

    // console::debug(format!("{}", color.render())); */
    kernel::shader::add(Box::new(shader::test_fs::test_fs));
    // let presult = color.render();
    loop {
        println!("{}", color.render());
    }
}
