use std::collections::HashMap;

use async_trait::async_trait;
use reqwest::Method;

use crate::api::methods::APIMethod;
use crate::api::methods::base::PayloadValue;
use crate::api::settings::Settings;

#[derive(Debug)]
pub struct PostMessageMethod<'a> {
    pub settings: &'a Settings,

    pub room_id: String,
    pub text: Option<String>,
    pub alias: Option<String>,
    pub emoji: Option<String>,
    pub avatar: Option<String>,
}

impl Default for PostMessageMethod<'_> {
    fn default() -> Self {
        static SETTINGS: Settings = Settings::None;

        PostMessageMethod {
            settings: &SETTINGS,
            room_id: "".to_string(),
            text: None,
            alias: None,
            emoji: None,
            avatar: None,
        }
    }
}

#[async_trait]
impl APIMethod for PostMessageMethod<'_> {
    fn settings(&self) -> &Settings {
        self.settings
    }

    fn endpoint(&self) -> &str {
        "/api/v1/chat.postMessage"
    }

    fn method(&self) -> Method {
        Method::POST
    }

    fn json_payload(&self) -> HashMap<String, PayloadValue> {
        let mut payload: HashMap<String, PayloadValue> = HashMap::new();
        payload.insert("roomId".to_string() , PayloadValue::String(&self.room_id));

        if let Some(text) = &self.text {
            payload.insert("text".to_string() , PayloadValue::String(&text));
        }
        if let Some(alias) = &self.alias {
            payload.insert("alias".to_string() , PayloadValue::String(&alias));
        }
        if let Some(emoji) = &self.emoji {
            payload.insert("emoji".to_string() , PayloadValue::String(&emoji));
        }
        if let Some(avatar) = &self.avatar {
            payload.insert("avatar".to_string() , PayloadValue::String(&avatar));
        }

        payload
    }
}
