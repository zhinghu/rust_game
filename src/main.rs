include!("core/Color.rs");
include!("core/Vec.rs");

fn main() {
    println!(
        "Version: {}-{}
Author: Github mychinesepyl",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );
    println!("");
    let v1: [f32; 2] = [f32::sin(0f32), f32::cos(0f32)];
    let v2: [f32; 2] = [f32::sin(90f32), f32::cos(90f32)];
    println!("Color: {:?}", vec::dot(&v1, &v2));
}
