use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

use std::env;

pub fn get_info() -> Result<(String, String, String), String> {
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

pub fn send(
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
