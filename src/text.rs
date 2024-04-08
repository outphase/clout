pub const MAIN_CPP: &str = "\
#include <iostream>

int main(){
    std::cout << \"Hello, World!\";
    return 0;
}\
";

pub const HELP: &str = "
||<><><><><><><><><><><><><><><><><><><><>||
||  Clout - A simple C++ project utility  ||
||<><><><><><><><><><><><><><><><><><><><>||
||                                        ||
||-------------------||-------------------||
||     -- Command -- || -- Use --         ||
||-------------------||-------------------||
||        new <name> || Create a project  ||
||                   || in a new <name>   ||
||                   || directory         ||
||-------------------||-------------------||
||        add <name> || Create a .cpp     ||
||                   || file with header  ||
||                   || in a the project  ||
||                   || directory         ||
||-------------------||-------------------||
||      build <mode> || Generate a Release||
||                   || build of your     ||
||                   || project           ||
||                   ||-------------------||
||                   || Modes:            ||;
||                   || -r -> Release     ||
||                   || -d -> Debug       ||
||-------------------||-------------------||
||        run <mode> || Generate a        ||
||                   || build of your     ||
||                   || project and run it||
||                   ||-------------------||
||                   || Modes:            ||
||                   || -r -> Release     ||
||                   || -d -> Debug       ||
||                   || -s -> Skip build  ||
||-------------------||-------------------||
||                                        ||
||<><><><><><><><><><><><><><><><><><><><>||\
";

pub const TITLE: &str = "\
||----||--------
||<><>|| Clout
||----||--------\
";

pub const WELCOME: &str = "\
||----||--------
||<><>|| WELCOME TO CLOUT
||----||--------
||<><>|| Please type `clout --help` for a list of commands\
";
