use crate::project;
use std::{
    fs,
    path::PathBuf,
    process::{self, Command},
};

pub enum BuildMode {
    Debug,
    Release,
}

pub fn build(mode: BuildMode) {
    let dbg = match mode {
        BuildMode::Debug => "--debug",
        BuildMode::Release => "",
    };
    let name = project::current_dir_name();
    let name = format!("{name}.exe");
    let file_names = get_compiler_file_names();
    println!("||<><>|| Compiling `{name}` with debug information.\n");

    let mut exe = Command::new("clang++");
    exe.current_dir("./build")
        .arg(dbg)
        .args(file_names)
        .arg("-o")
        .arg(name.trim());

    let thread = exe.spawn();
    if thread.is_err() {
        println!("||** Could not run clang++. Is it in PATH?");
        process::exit(1);
    }
    let thread = thread.unwrap();

    let output = thread.wait_with_output();
    if let Ok(output) = output {
        print!(
            "{}{}",
            String::from_utf8(output.stdout).expect("Could not parse stdout"),
            String::from_utf8(output.stderr).expect("Could not pars stderr")
        );
    } else {
        println!("||** WARNING\n||** Could not build `{name}`");
    };
}

fn get_compiler_files() -> Vec<PathBuf> {
    project::check_project_dir();
    fs::read_dir(".")
        .unwrap()
        .map(|res| res.map(|e| e.path()))
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap())
        .filter(|x| x.to_string_lossy().contains(".cpp"))
        .collect::<Vec<_>>()
}

fn get_compiler_file_names() -> Vec<String> {
    let mut file_names = Vec::new();
    for file in get_compiler_files() {
        let mut file_name = String::new();
        file_name.push_str("../");
        file_name.push_str(&file.file_name().unwrap().to_string_lossy());
        file_names.push(file_name);
    }
    file_names
}
