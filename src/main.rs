include!("core/Color.rs");

fn main() {
    println!(
        "Version: {}-{}
Author: Github mychinesepyl",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );
    println!("");
    let v1 = ndarray::arr1(&[1.0, 0.7]);
    let v2 = ndarray::arr1(&[0.0, -0.7]);

    println!("{:?}", v1.dot(&v2))
}
