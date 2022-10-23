#[derive(Debug)]
pub enum Error {
    JsonDecodeError(String),
    MissingSettings,
    RequestFailed(String),
    ResponseTextError,
}
