use todo_in_cli::Config;

#[test]
pub fn test_config_struct_success() {
    let args: Vec<String> = vec![
        String::from("target/debug/todo_in_cli"),
        String::from("list"),
    ];
    let config_object = Config::build(&args).unwrap();

    assert_eq!(config_object.command, "list");
}

#[test]
pub fn test_config_struct_failure() {
    let args: Vec<String> = vec![String::from("list")];
    let config_object = Config::build(&args);

    // assert!(config_object.is_err());

    assert_eq!(
        config_object.err(),
        Some(
            "Expected at least one CLI argument. Please run \"cargo run -- help\" to get more info"
        )
    );
}
