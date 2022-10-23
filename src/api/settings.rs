#[derive(Debug)]
pub struct LoginSettings {
    pub username: String,
    pub password: String,
    pub domain: String,
}

#[derive(Debug)]
pub struct AuthSettings {
    pub auth_token: String,
    pub user_id: String,
    pub domain: String
}

#[derive(Debug)]
pub enum Settings {
    None,
    Login(LoginSettings),
    Auth(AuthSettings),
}
