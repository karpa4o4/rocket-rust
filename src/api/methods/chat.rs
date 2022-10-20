use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::api::methods::base::{APIMethod, Payload};
use crate::api::Settings;

#[derive(Debug)]
pub struct PostMessageMethod<'a> {
    settings: &'a Settings,
    payload: PostMessagePayload,
}

#[derive(Serialize, Deserialize, Debug)]
struct PostMessagePayload {
    pub text: String,
    pub room_id: String,
}

impl Payload for PostMessagePayload {
    fn json(&self) -> HashMap<&str, &str> {
        let mut payload = HashMap::new();
        payload.insert("text" , self.text.as_str());
        payload.insert("roomId" , self.room_id.as_str());

        payload
    }
}

impl PostMessageMethod<'_> {
    pub fn new(settings: &Settings, text: String, room_id: String) -> PostMessageMethod {
        PostMessageMethod {
            settings,
            payload: PostMessagePayload {
                text,
                room_id,
            },
        }
    }
}

impl APIMethod for PostMessageMethod<'_> {
    fn settings(&self) -> &Settings {
        self.settings
    }

    fn endpoint(&self) -> &str {
        "/api/v1/chat.postMessage"
    }

    fn method(&self) -> reqwest::Method {
        reqwest::Method::POST
    }

    fn json_payload(&self) -> HashMap<&str, &str> {
        self.payload.json()
    }
}