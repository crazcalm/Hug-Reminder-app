use clap::builder::Arg;
use clap::Command;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

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
    let phone_number = env::var("PHONE_NUMBER")
        .expect("Unable to find 'PHONE_NUMBER' in the environment variables");
    let password = env::var("YAHOO_PASSWORD").unwrap();

    println!("email: {:?}", email);
    println!("phone_number: {:?}", phone_number);
    println!("password: {:?}", password);

    let email_message = Message::builder()
        .from(format!("Marcus Willock <{email}>").parse().unwrap())
        .to(
            format!("{} <{}@message.ting.com>", "Marcus Willock", phone_number)
                .parse()
                .unwrap(),
        )
        .subject("subject line")
        .body("Body line".to_string())
        .unwrap();

    let creds = Credentials::new(email, password);

    let mailer = SmtpTransport::relay("smtp.mail.yahoo.com")
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email_message) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }
}
