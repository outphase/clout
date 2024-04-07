pub const MAIN_CPP: &str = "\
#include <iostream>

int main(){
    std::cout << \"Hello, World!\";
    return 0;
}
";

pub const HELP: &str = "
||<><><><><><><><><><><><><><><><><><><><>||
||  Clout - A simple C++ project utility  ||
||<><><><><><><><><><><><><><><><><><><><>||
||                                        ||
||     -- Command -- || -- Use --         ||
||-------------------||-------------------||
||        new <name> || Create a project  ||
||                   || in a new <name>   ||
||                   || directory         ||
||-------------------||-------------------||
||      build <mode> || Generate a Release||
||                   || build of your     ||
||                   || project           ||
||                   ||-------------------||
||                   || Modes:            ||
||                   || -r -> Release     ||
||                   || -d -> Debug       ||
||                   ||                   ||
||                                        ||
||<><><><><><><><><><><><><><><><><><><><>||
";
