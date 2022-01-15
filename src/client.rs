use serde_json::json;
use reqwest::blocking::{Client as RQClient, Response, multipart};
use reqwest::header::{HeaderMap, HeaderValue, HeaderName};
use reqwest::Error;
use std::str::FromStr;


pub struct Client{
    protocol: String,
    base_uri: String,
    domain: String,
    token: String,
}

impl Client{
    pub fn new(protocol: &str, base_uri: &str, domain: &str, token: &str) -> Client {
        Self{
            protocol: protocol.to_string(),
            base_uri: base_uri.to_string(),
            domain: domain.to_string(),
            token: token.to_string(),
        }
    }
    pub fn send_simple_message(&self, from: &str, to: &str, subject: &str, text: &str) ->Result<Response, Error>{
        let url = format!("{}://{}/v3/{}/messages", self.protocol,
                          self.base_uri, self.domain);
        let form = multipart::Form::new()
            .text("from", from.to_string())
            .text("to", to.to_string())
            .text("subject", subject.to_string())
            .text("text", text.to_string());
        post_form(&url, &self.token, form)
    }
}

fn post(url: &str, token: &str, body: Option<String>)->Result<Response, Error>{
    println!("URL: {}", url);
    let mut header_map = HeaderMap::new();
    header_map.insert(HeaderName::from_str("Authorization").unwrap(),
                      HeaderValue::from_str(&format!("Basic {}", token)).unwrap());
    header_map.insert(HeaderName::from_str("Content-type").unwrap(),
                      HeaderValue::from_str("application/json").unwrap());
    let client = RQClient::builder()
        .default_headers(header_map)
        .build()
        .unwrap();
    match body{
        Some(content) => {
            println!("The content: {}", content);
            client.post(url).body(content).send()},
        None => client.post(url).send(),
    }
}

fn post_form(url: &str, token: &str, form: multipart::Form)->Result<Response, Error>{
    println!("URL: {}", url);
    let mut header_map = HeaderMap::new();
    header_map.insert(HeaderName::from_str("Authorization").unwrap(),
                      HeaderValue::from_str(&format!("Basic {}", token)).unwrap());
    let client = RQClient::builder()
        .default_headers(header_map)
        .build()
        .unwrap();
    client.post(url).multipart(form).send()
}
