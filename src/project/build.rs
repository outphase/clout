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
    println!("||<><>|| Compiling `{name}` with debug information.\n");

    dbg!(get_project_file_names());
    let mut exe = Command::new("clang++");
    exe.current_dir("./build")
        .arg(dbg)
        .args(get_project_file_names())
        .arg("-cxx-isystem")
        .args(get_project_dir_names())
        .arg("-o")
        .arg(name.trim());

    let thread = exe.spawn();
    if thread.is_err() {
        println!("||** Could not run clang++. Is it in PATH?");
        process::exit(1);
    }
    let thread = thread.unwrap();

    let output = thread.wait_with_output();
    match output {
        Ok(output) => {
            let stderr = String::from_utf8(output.stderr).expect("Could not pars stderr");
            let stdout = String::from_utf8(output.stdout).expect("Could not parse stdout");
            print!("{stdout}{stderr}");
            if output.status.to_string().trim() != "exit code: 0" || !stderr.is_empty() {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Compiler threw an error",
                ));
            }
        }
        Err(e) => {
            println!("||** WARNING\n||** Could not build `{name}`");
            return Err(e);
        }
    };
}

fn get_paths_at(path: &str) -> Vec<PathBuf> {
    project::check_project_dir();
    fs::read_dir(path)
        .unwrap()
        .map(|res| res.map(|e| e.path()))
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap())
        .collect()
}

fn get_project_dirs() -> Vec<PathBuf> {
    get_paths_at(".")
        .into_iter()
        .filter(|x| x.is_dir() && !x.file_name().unwrap().to_string_lossy().contains("build"))
        .collect()
}

fn get_project_dir_names() -> Vec<String> {
    let mut dir_names = Vec::new();
    for dir in get_project_dirs() {
        let mut dir_name = String::new();
        dir_name.push_str(".");
        dir_name.push_str(&dir.to_string_lossy());
        dir_names.push(dir_name);
    }
    dir_names
}

fn get_project_files() -> Vec<PathBuf> {
    let mut files = Vec::new();
    files.push(
        get_paths_at(".")
            .into_iter()
            .filter(|x| x.to_string_lossy().contains(".cpp"))
            .collect(),
    );
    for path in get_project_dirs() {
        files.append(
            &mut get_paths_at(path.to_str().unwrap())
                .into_iter()
                .filter(|x| x.to_string_lossy().contains(".cpp"))
                .collect::<Vec<PathBuf>>(),
        );
    }
    files
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
