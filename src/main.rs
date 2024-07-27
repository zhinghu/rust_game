include!("core/Color.rs");

fn main() {
    println!(
        "Version: {}-{}
Author: Github mychinesepyl
Use modules: ndarray(0.15.6)",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );
    println!("");
    let color = Color::new(&[-1.0, 1.0, 0.7, -0.3]);
    print!("Test Color: [");
    for elem in color.get() {
        print!("{},", elem);
    }
    print!("\x1b[1D]");
    println!("");
}
