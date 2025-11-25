use clap::Parser;
use wapp::cli::{Cli, Commands};

#[test]
fn test_parse_configure() {
    let cli = Cli::parse_from(vec!["wapp", "configure", "weatherapi"]);

    match cli.cmd {
        Commands::Configure { provider } => assert_eq!(provider, "weatherapi"),
        _ => panic!("wrong command parsed"),
    }
}

#[test]
fn test_parse_get() {
    let cli = Cli::parse_from(vec![
        "wapp", "get", "--city", "New York", "--data", "forecast",
    ]);

    match cli.cmd {
        Commands::Get { city, data } => {
            assert_eq!(city.unwrap(), "New York");
            assert_eq!(data, "forecast");
        }
        _ => panic!("wrong command parsed"),
    }
}
