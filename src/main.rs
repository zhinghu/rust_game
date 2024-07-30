include!("core/Color.rs");

fn main() {
    println!(
        "Version: {}-{}
Author: Github mychinesepyl",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );
    println!("");
    let color = Color::new(&[-1.0, 0.1, 0.7, -0.3]);
    println!("Color: {:?}", color.get());
}
