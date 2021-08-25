use reqwest;
use hyper::http::HeaderMap;
use hyper::http::header::AUTHORIZATION;
use base64;

pub struct Credentials {
    user: String,
    pass: String
}

impl Credentials {
    pub fn new(user: String, pass: String) -> Self {
        Self { user, pass }
    }
}

pub fn get_client(creds: &Credentials) -> reqwest::Client {
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION,
                   format!("Basic {}", base64::encode(format!("{}:{}", creds.user, creds.pass))
                   ).parse().unwrap());
    reqwest::Client::builder()
        .user_agent("integration-test/v0.1")
        .default_headers(headers)
        .build()
        .unwrap()
}
