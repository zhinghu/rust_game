#![allow(dead_code)]

mod kernel;
mod shader;

fn main() {
    println!(
        "Version: {}-{}
Author: Github mychinesepyl",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );
    println!("");
    let mut color = kernel::Render::new();
    kernel::shader::add(Box::new(shader::test), true);
    kernel::shader::add(Box::new(shader::tests), true);

    loop {
        color.render().unwrap();
    }
}
