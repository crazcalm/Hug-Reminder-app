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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::env;

    use super::*;

    fn setup_environment_variables() -> HashMap<&'static str, &'static str> {
        let map_values = HashMap::from([
            ("YAHOO_EMAIL", "fake_email"),
            ("YAHOO_PASSWORD", "fake_password"),
            ("PHONE_NUMBER_EMAIL", "fake_phone_number_email"),
        ]);

        for (key, value) in &map_values {
            env::set_var(key, value);
        }

        map_values
    }

    fn tear_down_environment_variables() {
        let keys = vec!["YAHOO_EMAIL", "YAHOO_PASSWORD", "PHONE_NUMBER_EMAIL"];

        for key in &keys {
            env::remove_var(key);
        }
    }

    /*
     * The test run in parallel, as such, setup from the other test causes this one to fail.
     *
    #[test]
    fn test_get_info_errors() {
        tear_down_environment_variables();

        let expected = "Unable to find YAHOO_EMAIL in environment variables, Unable to find YAHOO_PASSWORD in environment variables, Unable to find PHONE_NUMBER_EMAIL in environment variables".to_string();
        let result = get_info();

        assert!(result.is_err());

        assert_eq!(result.err().unwrap(), expected);
    }
    */

    #[test]
    fn test_get_info() {
        let map_values = setup_environment_variables();

        let (email, password, to_email) = get_info().unwrap();

        assert_eq!(
            email,
            map_values.get("YAHOO_EMAIL").unwrap().clone().to_string()
        );
        assert_eq!(
            password,
            map_values
                .get("YAHOO_PASSWORD")
                .unwrap()
                .clone()
                .to_string()
        );
        assert_eq!(
            to_email,
            map_values
                .get("PHONE_NUMBER_EMAIL")
                .unwrap()
                .clone()
                .to_string()
        );

        tear_down_environment_variables();
    }
}
