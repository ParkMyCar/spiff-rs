use actix_rt::System;
use actix_web::client::{Client, ClientBuilder, Connector};
use actix_web::web;
use futures::future::lazy;
use futures::future::Future;
use openssl::ssl::{SslConnector, SslMethod};
use serde_derive::{Deserialize, Serialize};
use std::fs;
use std::time::Duration;
use toml;
use twilio::twiml::{Action, Method, Sms};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
pub struct TwilioSMSRequestBody {
    ToCountry: Option<String>,
    ToState: Option<String>,
    SmsMessageSid: Option<String>,
    NumMedia: Option<String>,
    ToCity: Option<String>,
    FromZip: Option<String>,
    SmsSid: Option<String>,
    FromState: Option<String>,
    SmsStatus: Option<String>,
    FromCity: Option<String>,
    Body: Option<String>,
    FromCountry: Option<String>,
    To: Option<String>,
    ToZip: Option<String>,
    NumSegments: Option<String>,
    MessageSid: Option<String>,
    AccountSid: Option<String>,
    From: Option<String>,
    ApiVersion: Option<String>,
}
impl TwilioSMSRequestBody {
    pub fn body(&self) -> &Option<String> {
        &self.Body
    }

    pub fn from(&self) -> &Option<String> {
        &self.From
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct TwilioSendSMSForm {
    To: String,
    From: String,
    Body: String,
}
impl TwilioSendSMSForm {
    pub fn new(to: String, from: String, body: String) -> TwilioSendSMSForm {
        TwilioSendSMSForm {
            To: to,
            From: from,
            Body: body,
        }
    }
}

pub struct TwilioClient {
    account_sid: String,
    auth_token: String,

    client: Client,
}
impl TwilioClient {
    pub fn new(account_sid: String, auth_token: String) -> TwilioClient {
        let ssl_connector = SslConnector::builder(SslMethod::tls())
            .expect("Unable to build SSL connector!")
            .build();

        let connector = Connector::new()
            .ssl(ssl_connector)
            .timeout(Duration::from_secs(5))
            .finish();

        let client = ClientBuilder::default()
            .basic_auth(&account_sid, Some(auth_token.as_str()))
            .connector(connector)
            .finish();

        TwilioClient {
            account_sid,
            auth_token,
            client,
        }
    }

    pub fn send_sms(&self, to: String, body: String) {
        let base_url = format!(
            "https://api.twilio.com/2010-04-01/Accounts/{}/Messages.json",
            self.account_sid
        );

        let form_body = web::Form(TwilioSendSMSForm::new(
            to,
            String::from("+12028166496"),
            String::from("Hey Parker! Glad to be back 😊"),
        ))
        .into_inner();

        System::new("test")
            .block_on(lazy(|| {
                self.client
                    .post(base_url)
                    .send_form(&form_body)
                    .map_err(|err| (println!("{:?}", err)))
                    .and_then(|response| {
                        println!("{:?}", response);
                        Ok(())
                    })
            }))
            .unwrap();
    }
}

#[derive(Debug, Deserialize)]
pub struct TwilioKeys {
    pub account_sid: Option<String>,
    pub auth_token: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TwilioConfig {
    pub twilio: Option<TwilioKeys>,
}

pub fn read_config_file(config_filename: &str) -> Result<TwilioConfig, toml::de::Error> {
    let file_contents = fs::read_to_string(config_filename)
        .expect("Something went wrong with reading the config file!");
    toml::from_str(&file_contents)
}
