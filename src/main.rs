use clout::{self, text};
use std::{self, env, process};

fn main() {
    let command = env::args().nth(1).unwrap_or_else(|| {
        println!(" Please enter a command.\n Type clout --help for a list of commands.");
        process::exit(1);
    });

    let spec = env::args().nth(2);

    println!("\n||<><>||  Clout  ||<><>||");

    match command.trim() {
        "new" => {
            if let Some(spec) = spec {
                clout::project::new(spec.trim());
            } else {
                println!(" Please Provide a name for your project\n -> clout new <name>")
            }
        }
        "build" => {
            if let Some(spec) = spec {
                match spec.trim() {
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
            if let Some(spec) = spec {
                match spec.trim() {
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
    println!();
}
