pub mod text;

pub mod project {
    use crate::text;
    use std::{
        fs, io,
        path::Path,
        process,
        process::{Command, Output},
    };

    pub fn new(name: &str) {
        let _dir = fs::create_dir(&name);
        let _build_dir = fs::create_dir(format!("./{}/build", &name));
        let _file = fs::write(format!("./{}/main.cpp", name).trim(), text::MAIN_CPP);

        println!("||<><>|| Created project {name}");
    }

    pub fn debug() {
        let name = current_dir_name();
        println!("||<><>|| Compiling {name} with debug information.\n");
        let output = ps_command(
            format!("clang++ --debug ../main.cpp -o {name}.exe").trim(),
            "./build",
        );
        if let Ok(output) = output {
            print!(
                "{}",
                String::from_utf8(output.stdout).expect("||** Could not parse stdout")
            );
        } else {
            println!("||** WARNING\n||** Could not build {name}");
        };
    }

    pub fn release() {
        let name = current_dir_name();
        println!("||<><>|| Compiling {name}.\n");
        let output = ps_command(
            format!("clang++ ../main.cpp -o {name}.exe").trim(),
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
        // let output = exe.output();
        // let output = ps_command(format!("./build/{name}.exe").trim(), "");
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
}

#[cfg(test)]
mod tests {
    #[test]
    fn path_name() {
        let dir = std::env::current_dir().unwrap();
        let name = &dir.to_str().unwrap().split("\\").last().unwrap();

        assert_eq!(name, &"clout");
    }
}
