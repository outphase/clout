pub mod project;
mod tests;
pub mod text;

use crate::project::build::BuildMode;
use std;

pub fn run(command: String, spec: Option<String>) {
    println!("{}", text::TITLE);

    match command.trim() {
        "add" => {
            if let Some(spec) = spec {
                project::files::new_with_header(spec.trim());
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
            let build_result = if let Some(spec) = spec {
                choose_build(spec.trim())
            } else {
                project::build(BuildMode::Debug)
            };

            if let Ok(_) = build_result {
                project::run();
            }
        }

        "--help" => println!("{}", text::HELP),

        _ => {
            println!(
                "\
||** WARNING
||** Invalid Command.
||--------------
||** Please provide a valid command.
||** Type clout `--help` for a list of commands."
            );
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
