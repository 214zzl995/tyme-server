#[derive(Clone, Debug, Default)]
pub struct Store {
    api_token: String,
}

impl Store {
    pub fn new(api_token: String) -> Self {
        Self { api_token }
    }

    pub fn api_token_check(&self, auth_header: &str) -> bool {
        auth_header == format!("Bearer {}", self.api_token)
    }
}
