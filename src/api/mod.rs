use crate::api::methods::PostMessageMethod;
use crate::api::methods::{APIMethod, Payload};
use crate::errors::Error;

mod methods;

#[derive(Debug)]
pub struct Settings {
    pub username: String,
    pub password: String,
    pub domain: String,
}

#[derive(Debug)]
pub struct RocketChatAPI {
    settings: Settings,
}

impl RocketChatAPI {
    pub fn new(settings: Settings) -> RocketChatAPI {
        RocketChatAPI {
            settings,
        }
    }

    pub fn send_message(self, text: &str, room_id: &str) -> Result<String, Error> {
        let method = PostMessageMethod::new(
            &self.settings,
            String::from(text),
            String::from(room_id),
        );
        method.call()
    }
}
