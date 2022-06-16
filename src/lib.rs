pub mod cli;
pub mod db;
pub mod email;

/*
fn main() -> Result<(), String> {
    let cli_app = cli::get_app();

    let matches = cli_app.get_matches();

    let env_file_path = matches.get_one::<String>("env-file").unwrap();
    let _ = dotenv::from_filename(env_file_path).ok();

    let (email, password, to_email) = email::get_info()?;
    let subject = "Hug Reminder".to_string();
    let body = "Will fill out body later".to_string();

    if env_file_path.starts_with("dev") {
        println!("We do not send emails in dev");
    } else {
        let _ = email::send(email, password, to_email, subject, body)?;
    }
    Ok(())
}
*/
