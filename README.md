# || Welcome to Clout - A simple C++ project utility ||
Clout is an easy and simple C++ project manager, inspired by rust's [cargo](https://github.com/rust-lang/cargo), to use from the comfort of your command line.

## Usage

### Quick Start Guide

1. Download `clout.exe` from the [installation page]() and add it to your `PATH`.

2. Run the `clout new` command followed by the name of your new project.

3. Open the directory in your favourite text editor and code away!

### Commands

Here is a list of the commands. You can access this list by running the `clout --help` command.

| Command        | Use                                                   |
|----------------|-------------------------------------------------------|
| --help         | Show the list of commands                             |
| new \<name\>   | Create a project in a new ./\<name\> directory        |
| add \<name\>   | Create a .cpp file with header in a current directory |
| build \<mode\> | Generate a build of your project                      |
| run \<mode\>   | Generate a build of your project and run it           |

### Flags

|----|------------------------------------------------------------------------|
| -d | On any command that builds the project -> build with debug information |
| -r | On any command that builds the project -> build ready for release      |
| -s | On `clout run` -> skip the building step                               |

### Special files

If you type one of these special filenames when using the `clout add` command, you will create a preset file. You can find preset files [here]().

|----------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| \<filename\>.h | Create Header file only. Will have preset text.                                                                                           |
| random.h       | A usable, one file RNG library from [learncpp.com](https://www.learncpp.com/cpp-tutorial/introduction-to-random-number-generation/) |
