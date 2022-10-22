# rocketchat

### Rust API wrapper for the [RocketChat API](https://docs.rocket.chat/development-docs)

[![crates.io](https://img.shields.io/crates/v/rocketchat.svg)](https://crates.io/crates/rocketchat)
[![MIT/Apache-2 licensed](https://img.shields.io/crates/l/reqwest.svg)](./LICENSE-APACHE)

## Example

#### Initialize the client with a username and password.

```rust,no_run
use rocketchat::{LoginSettings, RocketChatAPI, Settings};

let settings = Settings::Login(LoginSettings {
    username: String::from("chuck_norris"),
    password: String::from("supersecret"),
    domain: String::from("https://mydomain.com"),
});
let client = RocketChatAPI::new(settings);
```

#### Initialize the client with an auth token and user ID.

```rust,no_run
use rocketchat::{AuthSettings, RocketChatAPI, Settings};

let settings = Settings::Auth(AuthSettings {
    auth_token: String::from("some_auth_token"),
    user_id: String::from("some_user_id"),
    domain: String::from("https://mydomain.com"),
});
let client = RocketChatAPI::new(settings);
```

#### Available API methods

##### [Post Message](https://developer.rocket.chat/reference/api/rest-api/endpoints/core-endpoints/chat-endpoints/postmessage)

```rust,no_run
let result = client.send_message("Some message with star emoji :star:", "#channel");
```


## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
