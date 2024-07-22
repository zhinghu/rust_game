include!("core/vec.rs");

fn main() {
    println!("Version: {}-{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    println!("Author: Github mychinesepyl\n");

    let test_vec_a: Vec<f32> = vec![-0.9, 0.0, 0.9, -1.0];
    let vec_rgb = vec::to_rgb(test_vec_a);

    println!("Color: {}, {}, {}, {}", vec_rgb[0], vec_rgb[1], vec_rgb[2], vec_rgb[3]);
}