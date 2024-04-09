use crate::text;
pub use build::{build, BuildMode};
pub use std::{
    fs, io,
    path::{Path, PathBuf},
    process::{self, Command, Output},
};

pub mod build;
pub mod files;

pub fn new(name: &str) {
    let _dir = fs::create_dir(&name);
    let _build_dir = fs::create_dir(format!("./{}/build", &name));
    let _file = fs::write(format!("./{}/main.cpp", name).trim(), text::MAIN_CPP);

    println!("||<><>|| Created project `{name}`");
}

pub fn run() {
    let name = current_dir_name();
    println!(
        "\
||--------||-------- 
||<><><><>|| Running `{name}.exe` \n"
    );
    let mut exe = Command::new(format!("./build/{name}.exe"));
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

fn check_project_dir() {
    if !Path::new("./main.cpp").exists() {
        println!("||** WARNING\n||** main.cpp not found. Is this a project directory?\n");
        process::exit(1);
    }
}

fn current_dir_name() -> String {
    check_project_dir();
    let dir = std::env::current_dir().unwrap();
    dir.to_str()
        .unwrap()
        .split("\\")
        .last()
        .unwrap()
        .to_string()
}
