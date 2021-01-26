use lettre_email::Email;
use lettre::{SmtpClient, SmtpTransport, Transport};
use actix_web::web::Json;
use crate::handlers::InputEmail;
use dotenv::dotenv;
use std::env;
use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::smtp::ConnectionReuseParameters;

pub fn email_sender(email: Json<InputEmail>) -> Result<(), MailSendingError> {
    let email = create_email(email);
    if let Err(_) = email {return Err(MailSendingError)}
    let email = email.unwrap();

    let mailer = get_mailer();
    if let Err(_) = mailer {return Err(MailSendingError)}
    let mut mailer = mailer.unwrap();


    match mailer.send(email.into()) {
        Ok(_) => Ok(()),
        Err(_) => Err(MailSendingError)
    }
}

fn get_mailer() -> Result<SmtpTransport, MailerError> {
    let (username, password) = get_email_account();
    let mailer = SmtpClient::new_simple("smtp.gmail.com");
    if mailer.is_err() {return Err(MailerError)}
    let mailer = mailer.unwrap()
        .credentials(Credentials::new(username, password))
        .smtp_utf8(true)
        .authentication_mechanism(Mechanism::Plain)
        .connection_reuse(ConnectionReuseParameters::ReuseUnlimited).transport();
    Ok(mailer)
} 

fn create_email(email: Json<InputEmail>) -> Result<Email, lettre_email::error::Error> {
    let (address, _) = get_email_account();

    Email::builder()
        .to(address)
        .from((email.from.clone(), email.name.clone()))
        .subject(&(email.subject))
        .text(&(email.text))
        .build()
}


fn get_email_account() -> (String, String) {
    dotenv().ok();

    let email = env::var("EMAIL_ACCOUNT").expect("EMAIL_ACCOUNT must be set");
    let vec: Vec<&str> = (&email).split("/").collect();
    (vec[0].to_owned(), vec[1].to_owned())
}


#[derive(Debug)]
struct MailerError;

impl std::fmt::Display for MailerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MailerError!")
    }
}

impl std::error::Error for MailerError {}

#[derive(Debug)]
pub struct MailSendingError;

impl std::fmt::Display for MailSendingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error while sending email!")
    }
}

impl std::error::Error for MailSendingError {}