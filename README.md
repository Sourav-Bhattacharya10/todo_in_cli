# TODO_IN_CLI
All the todos will be stored in TodoList.json

Following commands are available to run the application locally:
* cargo run -- list
* cargo run -- add "Buy Banana" "Buy Apple"
* cargo run -- done 0 1
* cargo run -- undone 0 2
* cargo run -- remove 2 3


To genearate test coverage:\
cargo tarpaulin --out Html\
\
To run a specific unit test:\
cargo test -p todo_in_cli test_config_struct_failure -- --nocapture
