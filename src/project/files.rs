use std::fs;

use crate::project;

pub fn new_with_header(name: &str) {
    project::check_project_dir();
    let _cpp = fs::write(format!("./{name}.cpp"), format!("#include \"{name}.h\""));
    let _h = fs::write(format!("./{name}.h"), "");
}
