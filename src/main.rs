use clap::builder::{App, Arg};
use clap::Command;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

use std::env;

fn get_app() -> App<'static> {
    Command::new("Hug-Reminder")
        .about("About section")
        .args(&[Arg::new("env-file")
            .short('e')
            .long("env_file")
            .default_value("dev.env")
            .help("Path to env file")])
}

fn get_info_for_email() -> Result<(String, String, String), String> {
    let wanted_variables = vec!["YAHOO_EMAIL", "YAHOO_PASSWORD", "PHONE_NUMBER_EMAIL"];
    let mut results: Vec<String> = vec![];
    let mut errors: Vec<String> = vec![];

    for variable in wanted_variables.iter() {
        let temp = env::var(variable);
        if let Ok(value) = temp {
            results.push(value);
        } else {
            errors.push(format!(
                "Unable to find {} in environment variables",
                &variable
            ));
        }
    }

    if !errors.is_empty() {
        let error_msg = errors.join(", ");
        return Err(error_msg);
    }

    let to_email = results.pop().unwrap();
    let password = results.pop().unwrap();
    let email = results.pop().unwrap();

    Ok((email, password, to_email))
}

fn send_email(
    email: String,
    password: String,
    to_email: String,
    subject: String,
    body: String,
) -> Result<(), String> {
    let email_message = Message::builder()
        .from(format!("Marcus Willock <{email}>").parse().unwrap())
        .to(format!("{} <{}>", "Marcus Willock", to_email)
            .parse()
            .unwrap())
        .subject(subject)
        .body(body)
        .unwrap();

    let creds = Credentials::new(email, password);

    let mailer = SmtpTransport::relay("smtp.mail.yahoo.com")
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email_message) {
        Ok(_) => {
            println!("Email was sent");
            Ok(())
        }
        Err(e) => Err(format!("Could not send email: {:?}", e)),
    }
}

fn main() -> Result<(), String> {
    let cli_app = get_app();

    let matches = cli_app.get_matches();

    let env_file_path = matches.get_one::<String>("env-file").unwrap();
    let _ = dotenv::from_filename(env_file_path).ok();

    let (email, password, to_email) = get_info_for_email()?;
    let subject = "Hug Reminder".to_string();
    let body = "Will fill out body later".to_string();

    if env_file_path.starts_with("dev") {
        println!("We do not send emails in dev");
    } else {
        let _ = send_email(email, password, to_email, subject, body)?;
    }
    Ok(())
}
