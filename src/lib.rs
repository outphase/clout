pub mod text;

pub mod project {
    use crate::text;
    use std::{
        fs, io,
        path::Path,
        process::{Command, Output},
    };

    pub fn new(name: &str) {
        let _dir = fs::create_dir(&name);
        let _build_dir = fs::create_dir(format!("./{}/build", &name));
        let _file = fs::write(format!("./{}/main.cpp", name).trim(), text::MAIN_CPP);
    }

    pub fn debug() {
        let name = current_dir_name();
        ps_command(
            format!("clang++ --debug ../main.cpp -o {name}.exe").trim(),
            "./build",
        )
        .expect("Could not build debug project");
    }

    pub fn release() {
        let name = current_dir_name();
        let output = ps_command(
            format!("clang++ ../main.cpp -o {name}.exe").trim(),
            "./build",
        )
        .expect("Could not build project");
        println!(
            "{}",
            String::from_utf8(output.stdout).expect("Could not parse stdout")
        );
    }

    pub fn run() {
        let name = current_dir_name();
        ps_command(format!("./build/{name}.exe").trim(), "").expect("Could not run build");
    }

    fn ps_command(command: &str, dir: &str) -> Result<Output, io::Error> {
        Command::new("powershell")
            .current_dir(format!("./{}", dir))
            .args(["/C", command])
            .output()
    }

    fn current_dir_name() -> String {
        assert!(Path::new("./main.cpp").exists());
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
