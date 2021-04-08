use std::ops::{Deref, DerefMut};

use isahc::{
    http::{Method, StatusCode},
    AsyncReadResponseExt as _, HttpClient, HttpClientBuilder,
};

use crate::{client_config::ClientConfig, error::Error};

#[derive(Debug)]
pub struct ClientBuilder {
    http_client_builder: HttpClientBuilder,
    client_config: ClientConfig,
}
impl Default for ClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl ClientBuilder {
    pub fn new() -> Self {
        Self {
            http_client_builder: HttpClientBuilder::new(),
            client_config: Default::default(),
        }
    }
    pub fn configurable<F>(mut self, func: F) -> Self
    where
        F: FnOnce(HttpClientBuilder) -> HttpClientBuilder,
    {
        self.http_client_builder = func(self.http_client_builder);
        self
    }
    pub fn build(self) -> Result<Client, Error> {
        Ok(Client {
            http_client: self.http_client_builder.build()?,
            client_config: self.client_config,
        })
    }
}
impl Deref for ClientBuilder {
    type Target = ClientConfig;

    fn deref(&self) -> &Self::Target {
        &self.client_config
    }
}
impl DerefMut for ClientBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.client_config
    }
}

#[derive(Debug, Clone)]
pub struct Client {
    http_client: HttpClient,
    client_config: ClientConfig,
}
impl Client {
    pub fn new() -> Result<Self, Error> {
        ClientBuilder::default().build()
    }
    pub async fn ping(&self) -> Result<bool, Error> {
        let (url, mut req) = self.client_config.get_url_and_request()?;
        let url = url.join("ping")?;

        *req.method_mut() = Method::GET;
        *req.uri_mut() = url.as_str().parse()?;

        let mut resp = self.http_client.send_async(req).await?;

        if resp.status() != StatusCode::OK {
            return Ok(false);
        }

        let resp_body_text = resp.text().await?;
        Ok(resp_body_text == "Ok.\n")
    }
}
