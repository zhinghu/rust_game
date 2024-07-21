include!("core/Color.rs");
include!("core/vec.rs");

fn main() {
    println!("Version: {}-{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    println!("Author: Github mychinesepyl\n");
}