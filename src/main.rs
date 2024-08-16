use ndarray::arr1;
mod kernel;
use kernel::console;

fn main() {
    println!(
        "Version: {}-{}
Author: Github mychinesepyl",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );
    println!("");
    let color = kernel::color::to_rgba(arr1(&[-1.0, 1.0, 1.0, 1.0]));
    let string_color = kernel::str::color_to_term(color);
    console::debug(format!("color string: {}", string_color));
    console::info(format!("0,255,255: {}", string_color));
}
