use std::collections::HashMap;

use reqwest::Method;

use crate::api::methods::APIMethod;
use crate::api::methods::base::PayloadValue;
use crate::api::settings::Settings;

pub struct ChannelCreateMethod<'a> {
    pub settings: &'a Settings,

    pub name: String,
    pub members: Option<Vec<String>>,
    pub read_only: Option<bool>,
}

impl Default for ChannelCreateMethod<'_> {
    fn default() -> Self {
        static SETTINGS: Settings = Settings::None;

        ChannelCreateMethod {
            settings: &SETTINGS,
            name: "".to_string(),
            members: None,
            read_only: Some(false),
        }
    }
}

impl APIMethod for ChannelCreateMethod<'_> {
    fn settings(&self) -> &Settings {
        self.settings
    }

    fn endpoint(&self) -> &str {
        "/api/v1/channels.create"
    }

    fn method(&self) -> Method {
        Method::POST
    }

    fn json_payload(&self) -> HashMap<String, PayloadValue> {
        let mut payload: HashMap<String, PayloadValue> = HashMap::new();
        payload.insert("name".to_string(), PayloadValue::String(&self.name));

        if let Some(members) = &self.members {
            let mem = members.into_iter().map(|el| el.as_ref()).collect();
            payload.insert("members".to_string(), PayloadValue::ListOfString(mem));
        }
        if let Some(read_only) = &self.read_only {
            payload.insert("readOnly".to_string(), PayloadValue::Bool(read_only));
        }

        payload
    }
}
