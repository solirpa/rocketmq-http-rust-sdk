use chrono::{DateTime, Utc};
use base64::{engine::general_purpose, Engine};
use log::debug;
use reqwest::{header::{self, HeaderMap}, Response, Method};
use sha1::Sha1;
use hmac::{Hmac, Mac};

use crate::{
    util::{get_canonicalized_mq_headers}, 
    error::ClientError, conf::ClientOption,
};

#[derive(Debug)]
pub(crate) struct Client {
    endpoint: String,
    access_key_id: String,
    access_key_secret: String,
    security_token: Option<String>,
    namespace: Option<String>,
}

impl Client {
    const X_MQ_VERSION: &'static str = "2015-06-06";
    const CONTENT_TYPE: &'static str = "text/xml;charset=utf-8";
    const USER_AGENT: &'static str = "mq-rust-sdk/1.0.4";
    const DEFAULT_HEADER_VALUE: &'static str = "";

    pub(crate) fn new(option: ClientOption) -> Self {
        Self {
            endpoint: option.endpoint,
            access_key_id: option.access_key_id,
            access_key_secret: option.access_key_secret,
            security_token: option.security_token,
            namespace: option.namespace,
        }
    }

    pub(crate) fn get_security_token(&self) -> Option<&str> {
        match &self.security_token {
            Some(token) => Some(token.as_str()),
            None => None,
        }
    }

    pub(crate) fn get_namespace(&self) -> Option<&str> {
        match &self.namespace {
            Some(namespace) => Some(namespace.as_str()),
            None => None,
        }
    }

    pub(crate) async fn get(&self, resource: &str) -> Result<Response, ClientError> {
        self.request(Method::GET, resource, None).await
    }

    pub(crate) async fn post(&self, resource: &str, body: String) -> Result<Response, ClientError> {
        self.request(Method::POST, resource, Some(body)).await
    }

    pub(crate) async fn delete(&self, resource: &str, body: String) -> Result<Response, ClientError> {
        self.request(Method::DELETE, resource, Some(body)).await
    }

    async fn request(&self, method: Method, resource: &str, body: Option<String>) -> Result<Response, ClientError>
    {
        let url: String = format!("{}{}", self.endpoint, resource);
        let mut headers: header::HeaderMap = header::HeaderMap::new();

        let now: DateTime<Utc> = Utc::now();
        let date = now.format("%a, %d %b %Y %H:%M:%S GMT").to_string();

        headers.insert(header::HeaderName::from_static("date"), header::HeaderValue::from_str(date.as_str()).unwrap());
        headers.insert(header::HeaderName::from_static("x-mq-version"), header::HeaderValue::from_static(Self::X_MQ_VERSION));
        headers.insert(header::HeaderName::from_static("content-type"), header::HeaderValue::from_static(Self::CONTENT_TYPE));
        headers.insert(header::HeaderName::from_static("user-agent"), header::HeaderValue::from_static(Self::USER_AGENT));

        let len;
        if let Some(body) = &body {
            len = body.len().to_string();
            let digest = md5::compute(body);
            let md5 = format!("{:?}", digest);
            let base64 = general_purpose::STANDARD.encode(&md5.as_bytes());

            headers.insert(header::HeaderName::from_static("content-length"), header::HeaderValue::from_str(len.as_str()).unwrap());
            headers.insert(header::HeaderName::from_static("content-md5"), header::HeaderValue::from_str(base64.as_str()).unwrap());
        }

        let sign = self.signature(&method, &headers, resource.as_ref())?;
        debug!("sign: {}", sign);

        headers.insert(header::HeaderName::from_static("authorization"), format!("MQ {}:{}", self.access_key_id, sign).parse().unwrap());
        if let Some(security_token) = self.get_security_token() {
            headers.insert(header::HeaderName::from_static("security-token"), header::HeaderValue::from_str(security_token).unwrap());
        }

        debug!("url: {}", url);
        debug!("headers: {:?}", headers);

        let client = reqwest::Client::new();
        let response = self.make_request(&client, url.as_str(), &method, headers, body).await?;

        Ok(response)
    }

    fn signature(&self, method: &Method, headers: &HeaderMap, resource: &str) ->  Result<String, ClientError> {
        let canonicalized_mq_headers = get_canonicalized_mq_headers(headers);
        let binding = header::HeaderValue::from_static(Self::DEFAULT_HEADER_VALUE);
        let md5 = headers.get("content-md5").unwrap_or(&binding).to_str().unwrap_or("");
        let date = headers.get("date").unwrap_or(&binding).to_str().unwrap_or("");
        let content_type = headers.get("content-type").unwrap_or(&binding).to_str().unwrap_or("");
        let to_sign_string = format!("{}\n{}\n{}\n{}\n{}{}", method, md5, content_type, date, canonicalized_mq_headers.as_str(), resource);
        debug!("to_sign_string: {}", to_sign_string);
        if let Ok(mut mac) = Hmac::<Sha1>::new_from_slice(self.access_key_secret.as_bytes()) {
            mac.update(to_sign_string.as_bytes());
    
            let digest = mac.finalize().into_bytes();
            Ok(general_purpose::STANDARD.encode(digest))
        } else {
            Err(ClientError::Signature)
        }
    }

    async fn make_request(
        &self,
        client: &reqwest::Client, url: &str,
        method: &Method, 
        headers: HeaderMap,
        body: Option<String>,
    ) -> Result<reqwest::Response, ClientError> {
        let builder = match *method {
            Method::GET => client.get(url),
            Method::DELETE => client.delete(url),
            Method::POST => client.post(url),
            _ => return Err(ClientError::UnsupportedMethod),
        };
        
        let builder = builder.headers(headers);
        let builder = if let Some(body) = body {
            builder.body(body)
        } else {
            builder
        };
        
        Ok(builder.send().await?)
    }
}
