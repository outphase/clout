use std::{fs, path::Path};

use rand::Rng;

use crate::{project, text};

pub fn add_random() {
    let name = "random.h";
    project::check_project_dir();
    if Path::exists(Path::new(format!("{name}").trim())) {
        println!("||** File `{name}` already exists");
        return;
    }
    let _h = fs::write(format!("./{name}"), text::files::RANDH);
    println!("||<><>|| Created preset `{name}` in project directory")
}

pub fn add_header(name: &str) {
    project::check_project_dir();
    if Path::exists(Path::new(format!("{name}").trim())) {
        println!("||** File `{name}` already exists");
        return;
    }
    write_header_file(name);
    println!("||<><>|| Created header `{name}` in project directory")
}

pub fn add_cpp_with_header(name: &str) {
    project::check_project_dir();
    if Path::exists(Path::new(format!("{name}.h").trim())) {
        println!("||** File `{name}.h` already exists");
        return;
    }

    write_header_file(name);
    write_cpp_file(name);
    println!("||<><>|| Created `{name}.cpp` and `{name}.h` in project directory")
}

fn write_header_file(name: &str) {
    let mut header_name: String = name.to_uppercase();
    header_name.push_str("_");
    header_name.push_str(format!("{}", rand::thread_rng().gen::<u16>()).trim());
    fs::write(
        format!("./{name}.h"),
        format!(
            "\
#ifndef {}
#define {}

//your code here

#endif",
            header_name, header_name,
        ),
    )
    .expect("||*** Could not create file");
}

fn write_cpp_file(name: &str) {
    fs::write(format!("./{name}.cpp"), format!("#include \"{name}.h\""))
        .expect("||*** Could not create file");
}
