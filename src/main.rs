mod kernel;
mod shader;
use std::time::Instant;

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
    loop {
        //     println!("{}", color.render());
        let a = Instant::now();
        let presult = color.render();
        let a = a.elapsed();
        console::debug(format!("render time: {}", a.as_secs_f32()));
    }
}
