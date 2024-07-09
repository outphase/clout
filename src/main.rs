use std::{env, process};

use clout::{
    project::{self, BuildMode},
    text,
};

fn main() {
    let command = env::args().nth(1).unwrap_or_else(|| {
        println!("{}", text::console::WELCOME);
        process::exit(1);
    });

    let spec = env::args().nth(2);

    run(command, spec);
}

fn run(command: String, spec: Option<String>) {
    println!("{}", text::console::TITLE);

    match command.trim() {
        "add" => command_add(spec),

        "new" => command_new(spec),

        "run" => command_run(spec),

        "build" => command_build(spec),

        "--help" => println!("{}", text::console::HELP),

        _ => println!("{}", text::console::INVALID_COMMAND),
    }

    println!("{}", text::console::ENDING);
}

fn command_run(spec: Option<String>) {
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

fn command_new(spec: Option<String>) {
    if let Some(spec) = spec {
        project::new(spec.trim());
    } else {
        println!("||** WARNING\n||** Please Provide a name for your project\n -> clout new <name>")
    }
}

fn command_add(spec: Option<String>) {
    if let Some(spec) = spec {
        project::add(spec.trim())
    } else {
        println!("||** WARNING\n||** Please Provide a name for the new file\n -> clout add <name>")
    }
}

fn command_build(spec: Option<String>) {
    let _ = if let Some(spec) = spec {
        choose_build(spec.trim())
    } else {
        project::build(BuildMode::Debug)
    };
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
