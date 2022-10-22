use std::collections::HashMap;

use reqwest::{Method};
use reqwest::blocking::{Client, Response};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::{Deserialize};

use crate::api::Settings;
use crate::errors::Error;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AuthData {
    pub user_id: String,
    pub auth_token: String,
}

#[derive(Deserialize, Debug)]
struct LoginResult {
    pub data: AuthData,
}

pub trait APIMethod {
    fn settings(&self) -> &Settings;
    fn endpoint(&self) -> &str;
    fn method(&self) -> Method;
    fn json_payload(&self) -> HashMap<&str, &str>;

    fn domain(&self) -> &String {
        match self.settings() {
            Settings::Auth(settings) => &settings.domain,
            Settings::Login(settings) => &settings.domain,
        }
    }

    fn build_endpoint(&self, uri: &str) -> String {
        format!("{}{}", self.domain(), uri)
    }

    fn request(
        &self,
        endpoint: String,
        method: Method,
        json_map: &HashMap<&str, &str>,
        auth_data: Option<AuthData>,
    ) -> Result<Response, Error> {
        let mut headers = HeaderMap::new();
        if let Some(data) = &auth_data {
            let auth_token_hdr: &'static str = "x-auth-token";
            headers.insert(
                HeaderName::from_static(auth_token_hdr),
                HeaderValue::from_str(data.auth_token.clone().as_str()).unwrap(),
            );

            let user_id_hdr: &'static str = "x-user-id";
            headers.insert(
                HeaderName::from_static(user_id_hdr),
                HeaderValue::from_str(data.user_id.clone().as_str()).unwrap(),
            );
        }

        let request = Client::default()
            .request(method, endpoint)
            .headers(headers)
            .json(&json_map);

        match request.send() {
            Ok(response) => Ok(response),
            Err(err) => {
                let msg = format!("{}", err);
                Err(Error::RequestFailed(msg))
            }
        }
    }

    fn login_payload<'a>(&'a self, username: &'a String, password: &'a String) -> HashMap<&str, &str> {
        let mut payload = HashMap::new();
        payload.insert("user", username.as_str());
        payload.insert("password", password.as_str());

        payload
    }

    fn login(&self, username: &String, password: &String) -> Result<AuthData, Error> {
        let response = self.request(
            self.build_endpoint("/api/v1/login"),
            Method::POST,
            &self.login_payload(username, password),
            None
        )?;

        if let Err(err) = response.error_for_status_ref() {
            let msg = format!("{}", err);
            return Err(Error::RequestFailed(msg));
        }

        let result: Result<LoginResult, _> = response.json();
        match result {
            Ok(login_result) => Ok(login_result.data),
            Err(err) => {
                let msg = format!("{}", err);
                Err(Error::JsonDecodeError(msg))
            }
        }
    }

    fn call(&self) -> Result<String, Error>{
        let auth_data= match self.settings() {
            Settings::Login(settings) => {
                // TODO: add processing and return LoginError
                self.login(&settings.username, &settings.password)?
            },
            Settings::Auth(settings) => AuthData{
                auth_token: settings.auth_token.clone(),
                user_id: settings.user_id.clone(),
            },
        };

        let response = self.request(
            self.build_endpoint(self.endpoint()),
            self.method(),
            &self.json_payload(),
            Some(auth_data)
        )?;

        if let Err(err) = response.error_for_status_ref() {
            let msg = format!("{}", err);
            return Err(Error::RequestFailed(msg));
        }

        match response.text() {
            Ok(text) => Ok(text),
            Err(_) => Err(Error::ResponseTextError),
        }
    }
}

pub trait Payload {
    fn json(&self) -> HashMap<&str, &str>;
}
