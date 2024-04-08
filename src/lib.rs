pub mod text;

pub mod project {
    use crate::text;
    use std::{
        fs, io,
        path::{Path, PathBuf},
        process::{self, Command, Output},
    };

    pub fn new(name: &str) {
        let _dir = fs::create_dir(&name);
        let _build_dir = fs::create_dir(format!("./{}/build", &name));
        let _file = fs::write(format!("./{}/main.cpp", name).trim(), text::MAIN_CPP);

        println!("||<><>|| Created project {name}");
    }

    pub fn debug() {
        let name = current_dir_name();
        let file_names = get_compiler_file_names();
        let command = format!("clang++ --debug {file_names} -o {name}.exe");
        let output = ps_command(&command.trim(), "./build");

        println!("AAAAAAAAAAAAAAAAAAAA {}", command);
        if let Ok(output) = output {
            print!(
                "{}",
                String::from_utf8(output.stdout).expect("||** Could not parse stdout")
            );
        } else {
            println!("||** WARNING\n||** Could not build {name}");
        };
        println!("||<><>|| Compiling {name} with debug information.\n");
    }

    pub fn release() {
        let name = current_dir_name();
        let file_names = get_compiler_file_names();
        let output = ps_command(
            format!("clang++ {file_names} -o {name}.exe").trim(),
            "./build",
        );

        if let Ok(output) = output {
            print!(
                "{}",
                String::from_utf8(output.stdout).expect("Could not parse stdout")
            );
        } else {
            println!("||** WARNING\n||** Could not build {name}");
        };
        println!("||<><>|| Compiling {name}.\n");
    }

    pub fn run() {
        let name = current_dir_name();
        println!(
            "\
||--------||-------- 
||<><><><>|| Running {name}.exe \n"
        );
        let mut exe = Command::new(format!("./build/{name}.exe"));
        let thread = exe.spawn();
        if let Ok(output) = thread.unwrap().wait_with_output() {
            print!(
                "{}",
                String::from_utf8(output.stdout).expect("could not parse stdout")
            );
        } else {
            println!("||** WARNING\n||** Could not run {name}.exe\n try building the project\n -> clout build")
        }
    }

    fn ps_command(command: &str, dir: &str) -> Result<Output, io::Error> {
        Command::new("powershell")
            .current_dir(format!("./{}", dir))
            .args(["/C", command])
            .output()
    }

    fn current_dir_name() -> String {
        if !Path::new("./main.cpp").exists() {
            println!("||** WARNING\n||** main.cpp not found. Is this a project directory?\n");
            process::exit(1);
        }
        let dir = std::env::current_dir().unwrap();
        dir.to_str()
            .unwrap()
            .split("\\")
            .last()
            .unwrap()
            .to_string()
    }

    fn get_compiler_files() -> Vec<PathBuf> {
        if !Path::new("./main.cpp").exists() {
            println!("||** WARNING\n||** main.cpp not found. Is this a project directory?\n");
            process::exit(1);
        }
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
}

#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn path_name() {
        let dir = std::env::current_dir().unwrap();
        let name = &dir.to_str().unwrap().split("\\").last().unwrap();

        assert_eq!(name, &"clout");
    }

    #[test]
    fn compiler_files() {
        let files: Vec<_> = fs::read_dir("./test")
            .unwrap()
            .map(|res| res.map(|e| e.path()))
            .filter(|x| x.is_ok())
            .map(|x| x.unwrap())
            .filter(|x| x.to_string_lossy().contains(".cpp"))
            .collect();

        assert_eq!(files[0].file_name().unwrap().to_string_lossy(), "main.cpp")
    }
}
