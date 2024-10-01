use std::fs;
use std::io::Write;
use std::path::Path;

fn main() {
    let out_dir = "./src/shaders/";
    let dest_path = Path::new(&out_dir).join("mod.rs");
    let mut mods_file = std::fs::File::create(&dest_path).unwrap();

    let files = fs::read_dir("./src/shaders/").unwrap();
    for file in files {
        let file = file.unwrap();
        if let Some(extension) = file.path().extension() {
            if extension == "rs" && file.file_name() != "mod.rs" {
                let mod_name = file
                    .file_name()
                    .to_str()
                    .unwrap()
                    .trim_end_matches(".rs")
                    .replace("-", "_");
                writeln!(
                    &mut mods_file,
                    "mod {};pub use {}::{};",
                    mod_name,
                    mod_name,
                    mod_name.replace("file", "")
                )
                .unwrap();
            }
        }
    }
}
