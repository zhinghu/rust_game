#![allow(dead_code)]

mod kernel;
mod shader;

fn main() {
    ctrlc::set_handler(|| {
        print!("\x1b[?47l\x1b[?25h");
        std::process::exit(0);
    })
    .unwrap();

    println!(
        "\x1b[?47h\x1b[?25l\x1b[0KVersion: {}-{}
Author: Github mychinesepyl",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );
    println!("");
    let mut color = kernel::Render::new();
    kernel::shader::add(Box::new(shader::test), true);

    loop {
        color.render().unwrap();
    }
}
