use std::fs;

use rand::Rng;

use crate::project;

pub fn new_with_header(name: &str) {
    project::check_project_dir();
    let mut header_name: String = name.to_uppercase();
    header_name.push_str("_");
    header_name.push_str(format!("{}", rand::thread_rng().gen::<i16>()).trim());
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
