use std::env;

use todo_in_cli::Config;


fn main() {
    let args = env::args().collect();
    let config_object = Config::build(&args).unwrap();
    todo_in_cli::run(config_object);
}
