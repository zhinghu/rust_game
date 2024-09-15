#![allow(dead_code)]

mod kernel;
mod shader;

use std::io::{self, Write};

fn main() -> io::Result<()> {
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
    let mut out = io::stdout();

    kernel::shader::add(Box::new(shader::test_fs::test_fs));
    out.write(color.render().as_bytes())?;
    out.flush()?;

    Ok(())
}
