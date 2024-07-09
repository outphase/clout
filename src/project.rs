use crate::text;
pub use build::{build, BuildMode};
pub use std::{
    fs, io,
    path::{Path, PathBuf},
    process::{self, Command, Output},
};

pub mod build;
pub mod files;
pub use files::add;

pub fn new(name: &str) {
    if Path::exists(Path::new(name)) {
        println!("||** Directory `{name}` already exists");
        return;
    }
    let _dir = fs::create_dir(&name);
    let _build_dir = fs::create_dir(format!("./{}/build", &name));
    let _cpp = fs::write(format!("./{}/main.cpp", name).trim(), text::MAIN_CPP);
    let _clangd = fs::write(format!("./{}/.clangd", name), text::CLANGD);

    println!("||<><>|| Created project `{name}`");
}

pub fn run(mode: BuildMode) {
    files::is_in_project_dir();
    let name = current_dir_name();
    println!(
        "
||----||------------ 
||<><>|| Running `{name}.exe`"
    );
    let path = match mode {
        BuildMode::Debug => format!("./build/{name}.exe"),
        BuildMode::Release => format!("./build/release/{name}.exe"),
    };
    let mut exe = Command::new(path);
    let thread = exe.spawn();
    match thread {
        Ok(thread) => {
            if let Ok(output) = thread.wait_with_output() {
                print!(
                    "{}",
                    String::from_utf8(output.stdout).expect("could not parse stdout")
                );
            } else {
                println!("||** WARNING\n||** Could not run `{name}.exe`\n try building the project\n -> clout build")
            }
        }
        Err(_) => {
            println!("||** WARNING\n||** Could not run `{name}.exe`\n try building the project\n -> clout build")
        }
    }
}

pub fn dir_name(path: &str) -> &str {
    path.split("\\").last().unwrap()
}

fn current_dir_name() -> String {
    files::is_in_project_dir();
    let dir = std::env::current_dir().unwrap();
    dir_name(dir.to_str().expect("Could not convert path to string")).to_string()
}
