use clout::{self, text};
use std::{self, env};

fn main() {
    let command = env::args()
        .nth(1)
        .expect(" Please enter a command.\n Type clout --help for a list of commands.");

    let spec = env::args().nth(2);

    match command.trim() {
        "new" => {
            let spec = spec.expect("Please specify a project name");
            clout::project::new(spec.trim());
        }
        "build" => {
            if let Some(r) = spec {
                match r.trim() {
                    "-r" => clout::project::release(),
                    "-d" => clout::project::debug(),
                    _ => {
                        println!("invalid argument, building debug.");
                        clout::project::debug();
                    }
                }
            } else {
                clout::project::debug();
            }
        }
        "run" => {
            if let Some(r) = spec {
                match r.trim() {
                    "-r" => clout::project::release(),
                    "-d" => clout::project::debug(),
                    "-s" => println!("Running older version."),
                    _ => {
                        println!("invalid argument, running debug.");
                        clout::project::debug();
                    }
                }
            } else {
                clout::project::debug();
            }
            clout::project::run();
        }
        "--help" => println!("{}", text::HELP),
        _ => {
            println!(
                " Please provide a valid command.\n Type clout --help for a list of commands."
            );
        }
    }
}
