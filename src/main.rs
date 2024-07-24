fn main() {
    println!("Version: {}-{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    println!("Author: Github mychinesepyl");
    println!("Use modules: ndarray(0.15.6)");
    println!("");

    let vec_a = ndarray::Array::from(vec![1.0, 0.0, 0.5]);
    let vec_b = ndarray::Array::from(vec![0.6, -0.2, -0.7]);

    println!("A dot B: {}", vec_a.dot(&vec_b));
}