pub mod text;

pub mod project {
    use crate::text;
    use std::{
        fs, io,
        path::Path,
        process::{Command, Output},
    };

    pub fn new(name: &str) {
        //ps_command(format!("mkdir {}", &name).trim(), "")
        //    .expect("Could not create project directory");

        // ps_command("New-Item main.cpp", &name).expect("Could not create main.cpp");
        // ps_command("mkdir build", &name).expect("Could not create build directory");

        let _dir = fs::create_dir(&name);
        let _build_dir = fs::create_dir(format!("./{}/build", &name));
        let _file = fs::write(format!("./{}/main.cpp", name).trim(), text::MAIN_CPP);
    }

    pub fn debug() {
        assert!(Path::new("./main.cpp").exists());

        let name = current_dir_name();
        ps_command(
            format!("clang++ --debug ../main.cpp -o {name}.exe").trim(),
            "./build",
        )
        .expect("Could not build debug project");
    }

    pub fn release() {
        assert!(Path::new("./main.cpp").exists());

        let name = current_dir_name();
        ps_command(
            format!("clang++ ../main.cpp -o {name}.exe").trim(),
            "./build",
        )
        .expect("Could not build project");
    }

    fn ps_command(command: &str, dir: &str) -> Result<Output, io::Error> {
        Command::new("powershell")
            .current_dir(format!("./{}", dir))
            .args(["/C", command])
            .output()
    }

    fn current_dir_name() -> String {
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
