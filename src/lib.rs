pub mod project;
mod tests;
pub mod text;

use crate::project::build::BuildMode;
use std;

pub fn run(command: String, spec: Option<String>) {
    println!("{}", text::console::TITLE);

    match command.trim() {
        "add" => {
            if let Some(spec) = spec {
                match spec.to_lowercase().trim() {
                    "random.h" => project::files::add_random(),
                    _ => {
                        if spec.contains(".h") {
                            project::files::add_header(spec.trim());
                        }
                        project::files::add_cpp_with_header(spec.trim());
                    }
                }
            } else {
                println!("||** WARNING\n||** Please Provide a name for the new file\n -> clout add <name>")
            }
        }

        "new" => {
            if let Some(spec) = spec {
                project::new(spec.trim());
            } else {
                println!("||** WARNING\n||** Please Provide a name for your project\n -> clout new <name>")
            }
        }

        "build" => {
            let _ = if let Some(spec) = spec {
                choose_build(spec.trim())
            } else {
                project::build(BuildMode::Debug)
            };
        }

        "run" => {
            if let Some(spec) = spec {
                if spec.trim() == "-s" {
                    project::run(BuildMode::Debug);
                } else {
                    if let Ok(_) = choose_build(&spec) {
                        project::run(match spec.trim() {
                            "-r" => BuildMode::Release,
                            _ => BuildMode::Debug,
                        })
                    }
                }
            } else {
                if let Ok(_) = project::build(BuildMode::Debug) {
                    project::run(BuildMode::Debug);
                } else {
                    println!("||** Could not build project, not running")
                }
            }
        }

        "--help" => println!("{}", text::console::HELP),

        _ => {
            println!("{}", text::console::INVALID_COMMAND);
        }
    }
    println!(
        "
||<><>||
||----||------------
"
    );
}

fn choose_build(spec: &str) -> std::io::Result<()> {
    match spec {
        "-r" => project::build(BuildMode::Release),
        "-d" => project::build(BuildMode::Debug),
        _ => {
            println!("||** invalid argument, building debug.");
            project::build(BuildMode::Debug)
        }
    }
}
