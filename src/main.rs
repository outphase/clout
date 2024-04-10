use std::{env, process};

use clout::{run, text};

fn main() {
    let command = env::args().nth(1).unwrap_or_else(|| {
        println!("{}", text::WELCOME);
        process::exit(1);
    });

    let spec = env::args().nth(2);

    run(command, spec);
}
