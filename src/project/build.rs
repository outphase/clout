use crate::project;
use std::{fs, path::PathBuf};

pub fn debug() {
    let name = project::current_dir_name();
    let file_names = get_compiler_file_names();
    let command = format!("clang++ --debug {file_names} -o {name}.exe");
    let output = project::ps_command(&command.trim(), "./build");

    if let Ok(output) = output {
        print!(
            "{}",
            String::from_utf8(output.stdout).expect("||** Could not parse stdout")
        );
    } else {
        println!("||** WARNING\n||** Could not build `{name}`");
    };
    println!("||<><>|| Compiling `{name}` with debug information.\n");
}

pub fn release() {
    let name = project::current_dir_name();
    let file_names = get_compiler_file_names();
    let command = format!("clang++ {file_names} -o {name}.exe");
    let output = project::ps_command(&command.trim(), "./build");

    if let Ok(output) = output {
        print!(
            "{}",
            String::from_utf8(output.stdout).expect("Could not parse stdout")
        );
    } else {
        println!("||** WARNING\n||** Could not build `{name}`");
    };
    println!("||<><>|| Compiling `{name}`.\n");
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

fn get_compiler_file_names() -> String {
    let mut file_names = String::new();
    for file in get_compiler_files() {
        file_names.push_str("../");
        file_names.push_str(&file.file_name().unwrap().to_string_lossy());
        file_names.push_str(" ");
    }
    file_names
}
