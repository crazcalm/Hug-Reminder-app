use clap::builder::{App, Arg};
use clap::Command;

pub fn get_app() -> App<'static> {
    Command::new("Hug-Reminder")
        .about("About section")
        .args(&[Arg::new("env-file")
            .short('e')
            .long("env_file")
            .default_value("dev.env")
            .help("Path to env file")])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_app_defaults() {
        let args: Vec<&str> = vec![];
        let matches = get_app().get_matches_from(args);
        let env_file_path = matches.get_one::<String>("env-file").unwrap();
        assert_eq!(env_file_path, "dev.env");
    }

    #[test]
    fn test_get_app_with_env_file_path() {
        let args = vec!["my_prog", "-e", "testing.env"];
        let matches = get_app().get_matches_from(args);
        let env_file_path = matches.get_one::<String>("env-file").unwrap();
        assert_eq!(env_file_path, "testing.env");
    }
}
