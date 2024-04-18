pub const HELP: &str = "
||<><><><><><><><><><><><><><><><><><><><>||
||  Clout - A simple C++ project utility  ||
||<><><><><><><><><><><><><><><><><><><><>||
||                                        ||
||-------------------||-------------------||
||     -- Command -- || -- Use --         ||
||-------------------||-------------------||
||        new <name> || Create a project  ||
||                   || in a new ./<name> ||
||                   || directory         ||
||-------------------||-------------------||
||        add <name> || Create a .cpp     ||
||                   || file with header  ||
||                   || in a the current  ||
||                   || directory         ||
||                   ||-------------------||
||                   || Special Cases:    ||
||                   || <filename>.h ->   ||
||                   ||     create header ||
||                   ||     file only     ||
||                   || random.h ->       ||
||                   ||     create preset ||
||                   ||     random.h file ||
||-------------------||-------------------||
||      build <mode> || Generate a debug  ||
||                   || build of your     ||
||                   || project           ||
||                   ||-------------------||
||                   || Modes:            ||
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

pub const TITLE: &str = "
||----||--------
||<><>|| Clout
||----||--------\
";

pub const WELCOME: &str = "
||----||--------
||<><>|| WELCOME TO CLOUT
||----||--------
||<><>|| Please type `clout --help` for a list of commands\
";

pub const INVALID_COMMAND: &str = "\
||** WARNING
||** Invalid Command.
||--------------
||** Please provide a valid command.
||** Type `clout --help` for a list of commands.\
";
