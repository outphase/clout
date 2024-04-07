use clout::{self, text};
use core::panic;
use std::{self, env, process::Command};

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
                        println!("Invalid argument, building debug.");
                        clout::project::debug();
                    }
                }
            } else {
                clout::project::debug();
            }
        }
        "--help" => println!("{}", text::HELP),
        _ => {
            println!(
                " Please provide a valid command.\n Type clout --help for a list of commands."
            );
        }
    }

    let output = Command::new("powershell")
        .args(["/C", "echo Hello"])
        .output()
        .expect("Could not execute command");

    println!("{}", String::from_utf8(output.stdout).unwrap());
}
