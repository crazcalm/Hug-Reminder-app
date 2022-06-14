use clap::builder::Arg;
use clap::Command;

use std::env;

fn main() {
    let cli_app = Command::new("Hug-Reminder")
        .about("About section")
        .args(&[Arg::new("env-file")
            .short('e')
            .long("env_file")
            .default_value("dev.env")
            .help("Path to env file")]);

    let matches = cli_app.get_matches();

    let env_file_path = matches.get_one::<String>("env-file").unwrap();
    println!("{:?}", env_file_path);

    println!("matches: {:?}", matches);

    let _ = dotenv::from_filename(env_file_path).ok();

    let email =
        env::var("YAHOO_EMAIL").expect("Unable to find 'YAHOO_EMAIL' in the environment variables");

    println!("email: {:?}", email);
}
