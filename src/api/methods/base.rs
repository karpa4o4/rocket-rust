use std::collections::HashMap;

use async_trait::async_trait;
use reqwest::{Client, Method, Response};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::Deserialize;

use crate::api::settings::Settings;
use crate::errors::Error;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AuthData {
    pub user_id: String,
    pub auth_token: String,
}

#[derive(Debug, Deserialize)]
struct LoginResult {
    pub data: AuthData,
}

#[async_trait]
pub trait APIMethod {
    fn settings(&self) -> &Settings;
    fn endpoint(&self) -> &str;
    fn method(&self) -> Method;
    fn json_payload(&self) -> HashMap<String, &str>;

    fn domain(&self) -> Result<&str, Error> {
        match self.settings() {
            Settings::None => Err(Error::MissingSettings),
            Settings::Auth(settings) => Ok(&settings.domain),
            Settings::Login(settings) => Ok(&settings.domain),
        }
    }

    fn build_endpoint(&self, uri: &str) -> Result<String, Error> {
        Ok(format!("{}{}", self.domain()?, uri))
    }

    async fn request(
        &self,
        endpoint: String,
        method: Method,
        json_map: &HashMap<String, &str>,
        auth_data: Option<AuthData>,
    ) -> Result<Response, Error> {
        let mut headers = HeaderMap::new();
        if let Some(data) = &auth_data {
            let auth_token_hdr: &str = "x-auth-token";
            headers.insert(
                HeaderName::from_static(auth_token_hdr),
                HeaderValue::from_str(data.auth_token.clone().as_str()).unwrap(),
            );

            let user_id_hdr: &str = "x-user-id";
            headers.insert(
                HeaderName::from_static(user_id_hdr),
                HeaderValue::from_str(data.user_id.clone().as_str()).unwrap(),
            );
        }

        let request = Client::default()
            .request(method, endpoint)
            .headers(headers)
            .json(&json_map);

        match request.send().await {
            Ok(response) => Ok(response),
            Err(err) => {
                let msg = err.to_string();
                Err(Error::RequestFailed(msg))
            }
        }
    }

    fn login_payload<'a>(&'a self, username: &'a str, password: &'a str) -> HashMap<String, &str> {
        let mut payload = HashMap::new();
        payload.insert("user".to_string(), username);
        payload.insert("password".to_string(), password);

        payload
    }

    async fn login(&self, username: &str, password: &str) -> Result<AuthData, Error> {
        let response = self.request(
            self.build_endpoint("/api/v1/login")?,
            Method::POST,
            &self.login_payload(username, password),
            None
        ).await?;

        if let Err(err) = response.error_for_status_ref() {
            let msg = err.to_string();
            return Err(Error::RequestFailed(msg));
        }

        let result: Result<LoginResult, _> = response.json().await;
        match result {
            Ok(login_result) => Ok(login_result.data),
            Err(err) => {
                let msg = err.to_string();
                Err(Error::JsonDecodeError(msg))
            }
        }
    }

    async fn call(&self) -> Result<String, Error> {
        let auth_data = match self.settings() {
            Settings::None => Err(Error::MissingSettings),
            Settings::Login(settings) => {
                // TODO: add processing and return LoginError
                Ok(self.login(&settings.username, &settings.password).await?)
            },
            Settings::Auth(settings) => Ok(
                AuthData{
                    auth_token: settings.auth_token.clone(),
                    user_id: settings.user_id.clone(),
                }
            ),
        }?;

        let response = self.request(
            self.build_endpoint(self.endpoint())?,
            self.method(),
            &self.json_payload(),
            Some(auth_data)
        ).await?;

        if let Err(err) = response.error_for_status_ref() {
            let msg = err.to_string();
            return Err(Error::RequestFailed(msg));
        }

        match response.text().await {
            Ok(text) => Ok(text),
            Err(_) => Err(Error::ResponseTextError),
        }
    }
}
