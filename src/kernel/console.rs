pub fn debug(text: String) {
    println!("[\x1b[38;2;0;255;0mDEBUG\x1b[0m] {:?}", text);
}

pub fn info(text: String) {
    println!("[\x1b[38;2;0;255;255mINFO\x1b[0m] {}", text);
}

pub fn err(text: String) {
    println!("[\x1b[38;2;255;0;0mERROR\x1b[0m] {}", text);
}

pub fn warn(text: String) {
    println!("[\x1b[38;2;255;255;0mWARN\x1b[0m] {}", text);
}
