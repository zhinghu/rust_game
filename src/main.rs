use ndarray::arr1;

mod kernel;

fn main() {
    println!(
        "Version: {}-{}
Author: Github mychinesepyl",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );
    println!("");
    println!("{:?}", kernel::color::to_rgba(arr1(&[-1.0, 2.0, 0.7, 0.0])));
}
