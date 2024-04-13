use std::{fs, path::Path};

use rand::Rng;

use crate::project;

pub fn new_with_header(name: &str) {
    project::check_project_dir();
    if Path::exists(Path::new(format!("{name}.h").trim())) {
        println!("||** File `{name}.h` already exists");
        return;
    }
    let mut header_name: String = name.to_uppercase();
    header_name.push_str("_");
    header_name.push_str(format!("{}", rand::thread_rng().gen::<u16>()).trim());
    let _cpp = fs::write(format!("./{name}.cpp"), format!("#include \"{name}.h\""));
    let _h = fs::write(
        format!("./{name}.h"),
        format!(
            "\
#ifndef {}
#define {}

//your code here

#endif",
            header_name, header_name,
        ),
    );
    println!("||<><>|| Created `{name}.cpp` and `{name}.h` in project directory")
}
