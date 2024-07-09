# || Welcome to Clout - A simple C++ project utility ||
Clout is an easy and simple C++ project manager, inspired by rust's [cargo](https://github.com/rust-lang/cargo), to use from the comfort of your command line.

## Usage

Here is a list of the commands. You can access this list by running `clout --help`.

| Command      | Use                |
|--------------|--------------------|
| new <name>   | Create a project   |
|              | in a new ./<name>  |
|              | directory          |
| add <name>   | Create a .cpp      |
|              | file with header   |
|              | in a the current   |
|              | directory          |
|              | Special Cases:     |
|              | <filename>.h ->    |
|              | create header      |
|              | file only          |
|              | random.h ->        |
|              | create preset      |
|              | random.h file      |
| build <mode> | Generate a debug   |
|              | build of your      |
|              | project            |
|              | Modes:             |
|              | -r -> Release      |
|              | -d -> Debug        |
| run <mode>   | Generate a         |
|              | build of your      |
|              | project and run it |
|              | Modes:             |
|              | -r -> Release      |
|              | -d -> Debug        |
|              | -s -> Skip build   |
