#[derive(Debug)]
pub enum Error {
    JsonDecodeError(String),
    RequestFailed(String),
    ResponseTextError,
}
