use std::fs;
use std::io::Write;
use std::path::Path;

fn main() {
    let out_dir = "./src/shaders/";
    let dest_path = Path::new(&out_dir).join("mod.rs");
    let mut mods_file = std::fs::File::create(&dest_path).unwrap();
    writeln!(
        &mut mods_file,
        "{}",
        "use std::sync::Once;

lazy_static::lazy_static! {
        static ref CALLED_ONCE: Once = Once::new();
}\n"
    )
    .unwrap();

    let files: Vec<_> = fs::read_dir("./src/shaders/")
        .unwrap()
        .map(|result| result.unwrap())
        .collect();

    for file in &files {
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
                    "mod {};use {}::{};",
                    &mod_name,
                    &mod_name,
                    &mod_name.replace("file", "")
                )
                .unwrap();
            }
        }
    }

    writeln!(
        &mut mods_file,
        "{}",
        "\npub fn init() {CALLED_ONCE.call_once(|| {"
    )
    .unwrap();

    for file in &files {
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
                    "\u{0020}\u{0020}\u{0020}\u{0020} super::kernel::shader::add({}, \"{}\", {}, {}::SHADER_ACTIVE);",
                    if mod_name.len() >= 2 {
                        if &mod_name[..2] == "vs" {
                            "super::kernel::shader::Shader::VertexShader"
                        } else {
                            "super::kernel::shader::Shader::FragmentShader"
                        }
                    } else {
                        ""
                    },
                    &mod_name.replace("file", "")[3..],
                    &mod_name.replace("file", ""),
                    &mod_name
                )
                .unwrap();
            }
        }
    }

    writeln!(&mut mods_file, "{}", "});}").unwrap();
}
