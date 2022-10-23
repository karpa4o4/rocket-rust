# rocketchat

### Rust API wrapper for the [RocketChat API](https://docs.rocket.chat/development-docs)

[![crates.io](https://img.shields.io/crates/v/rocketchat.svg)](https://crates.io/crates/rocketchat)
[![MIT](https://img.shields.io/crates/l/rocketchat.svg)](./LICENSE)

## Example

The library uses asynchronous HTTP client [reqwest](https://crates.io/crates/reqwest), so your Cargo.toml could look like this:

```toml
rocketchat = "0.4.0"
tokio = { version = "1", features = ["full"] }
```

### When calling methods, you need to pass settings that can be created as follows:

#### Using username and password

```rust,no_run
use rocketchat::{LoginSettings, Settings};

let settings = Settings::Login(LoginSettings {
    username: "chuck_norris".to_string(),
    password: "supersecret".to_string(),
    domain: "https://mydomain.com".to_string(),
});
```

#### Using auth token and user ID

```rust,no_run
use rocketchat::{AuthSettings, Settings};

let settings = Settings::Auth(AuthSettings {
    auth_token: "some_auth_token".to_string(),
    user_id: "some_user_id".to_string(),
    domain: "https://mydomain.com".to_string(),
});
```

### Available API methods

#### [Post Message](https://developer.rocket.chat/reference/api/rest-api/endpoints/core-endpoints/chat-endpoints/postmessage)

```rust,no_run
let result = PostMessageMethod {
    settings: &settings,
    room_id: "#channel".to_string(),
    text: Some("Some message with star emoji :star:".to_string()),
    ..Default::default()
}.call().await;
```

## License

MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
