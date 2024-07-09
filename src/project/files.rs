use std::{fs, path::Path, process};

use rand::Rng;

use crate::text;

const PRESET_FILE_NAMES: [&str; 1] = ["random.h"];

pub fn add(spec: &str) {
    is_in_project_dir();

    /* TODO: Prefrences and user customizable template files
        let read_dir = fs::read_dir("./template files/").expect("Could not read dir");
        for file in read_dir {
            if let Ok(file) = file {
                let file_path = file.path();
                let file_name = dir_name(file_path.to_str().expect("Could not convert path to str"));
                if file_name == spec {
                    if Path::exists(Path::new(format!("{file_name}").trim())) {
                        println!("||** File `{file_name}` already exists");
                        return;
                    }
                    let _new_file = fs::copy(&file_path, format!("./{}", &file_name));
                    println!("||<><>|| Created preset `{file_name}` in project directory");
                    return;
                }
            }
        }
    */

    if !PRESET_FILE_NAMES.contains(&spec) {
        if spec.contains(".h") {
            add_header(spec.trim());
        }
        add_cpp_with_header(spec.trim());
    }

    match spec.trim() {
        "random.h" => add_preset_file("random.h", text::RANDH),
        _ => eprintln!("Shouldn't have gotten to this point..."),
    }
}

pub fn is_in_project_dir() {
    if !Path::new("./main.cpp").exists() {
        println!("||** WARNING\n||** main.cpp not found. Is this a project directory?\n");
        process::exit(1);
    }
}

pub fn add_preset_file(name: &str, content: &str) {
    is_in_project_dir();
    if Path::exists(Path::new(format!("{name}").trim())) {
        println!("||** File `{name}` already exists");
        return;
    }
    let _h = fs::write(format!("./{name}"), content);
    println!("||<><>|| Created preset `{name}` in directory")
}

pub fn add_header(name: &str) {
    is_in_project_dir();
    if Path::exists(Path::new(format!("{name}").trim())) {
        println!("||** File `{name}` already exists");
        return;
    }
    write_header_file(name);
    println!("||<><>|| Created header `{name}` in irectory")
}

pub fn add_cpp_with_header(name: &str) {
    is_in_project_dir();
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
    .expect("||*** Could not create .h file");
}

fn write_cpp_file(name: &str) {
    fs::write(format!("./{name}.cpp"), format!("#include \"{name}.h\""))
        .expect("||*** Could not create .cpp file");
}

#[cfg(test)]
mod tests {
    #[allow(unused)]
    use core::panic;
    use std::fs;

    #[test]
    fn files_in_dir() {
        let read_dir = fs::read_dir("./template files/").expect("Could not read dir");
        for file in read_dir {
            if let Ok(file) = file {
                println!("{:?}", file)
            }
        }
        // panic!("")
    }
}
