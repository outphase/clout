use crate::project;
use std::{
    path::PathBuf,
    process::{self, Command},
};

pub enum BuildMode {
    Debug,
    Release,
}

pub fn build(mode: BuildMode) -> std::io::Result<()> {
    let dbg = match mode {
        BuildMode::Debug => "--debug",
        BuildMode::Release => "",
    };
    let name = project::current_dir_name();
    let name = format!("./{name}.exe");
    let files = get_project_file_names();
    let dirs = get_project_dir_names();
    println!("||<><>|| Compiling `{name}` with debug information.");
    for res in dirs.iter() {
        println!("||<> Searching in `{}`", &res[1..]);
    }
    println!("||<>");
    for res in files.iter() {
        println!("||<> Processing `{}`", &res[3..]);
    }

    let mut exe = Command::new("clang++");
    exe.current_dir("./build")
        .arg(dbg)
        .args(files)
        .args(["-o", name.trim()]);
    if !dirs.is_empty() {
        for d in dirs {
            exe.arg("-cxx-isystem").arg(d);
        }
    }
    // dbg!(&exe.get_args().collect::<Vec<&std::ffi::OsStr>>());

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
            println!("{stdout}{stderr}");
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
    Ok(())
}

fn get_project_paths() -> Vec<PathBuf> {
    walkdir::WalkDir::new(".")
        .into_iter()
        .filter(|x| x.is_ok())
        .map(|x| PathBuf::from(x.unwrap().path()))
        .collect::<Vec<PathBuf>>()
}

fn get_project_dirs() -> Vec<PathBuf> {
    get_project_paths()
        .into_iter()
        .filter(|x| x.is_dir() && !x.to_string_lossy().contains("build"))
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
    files.append(
        &mut get_project_paths()
            .into_iter()
            .filter(|x| x.to_string_lossy().contains(".cpp"))
            .collect(),
    );
    files
}

fn get_project_file_names() -> Vec<String> {
    let mut file_names = Vec::new();
    for file in get_project_files() {
        let mut file_name = String::new();
        file_name.push_str("../");
        file_name.push_str(&file.to_string_lossy());
        file_names.push(file_name);
    }
    file_names
}
