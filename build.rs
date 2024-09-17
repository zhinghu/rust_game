use std::{env, process::Command};

fn main() {
    let CC = env::var("CC").expect("Not found CC environment");
    Command::new(CC).arg("-c").arg("");
}
