pub mod project;
mod tests;
pub mod text;

use std;

pub fn run(command: String, spec: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", text::TITLE);

    match command.trim() {
        "new" => {
            if let Some(spec) = spec {
                project::new(spec.trim());
            } else {
                println!("||** WARNING\n||** Please Provide a name for your project\n -> clout new <name>")
            }
        }
        "build" => {
            if let Some(spec) = spec {
                match spec.trim() {
                    "-r" => project::build::release(),
                    "-d" => project::build::debug(),
                    _ => {
                        println!("||** invalid argument, building debug.");
                        project::build::debug();
                    }
                }
            } else {
                project::build::debug();
            }
        }
        "run" => {
            if let Some(spec) = spec {
                match spec.trim() {
                    "-r" => project::build::release(),
                    "-d" => project::build::debug(),
                    "-s" => println!("||!! Running older version. "),
                    _ => {
                        println!("||** invalid argument, running debug.");
                        project::build::debug();
                    }
                }
            } else {
                project::build::debug();
            }
            project::run();
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
    Ok(())
}
