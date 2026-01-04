use core::ops::{Deref, DerefMut};

use clickhouse_format::{format_name::FormatName, input::Input, output::Output};
use isahc::{
    AsyncBody, AsyncReadResponseExt as _, HttpClient, HttpClientBuilder,
    http::{Method, Request, Response, StatusCode, response::Parts as ResponseParts},
};

use crate::{
    client_config::{
        ClientConfig, FORMAT_KEY_HEADER, FORMAT_KEY_URL_PARAMETER, QUERY_KEY_URL_PARAMETER,
    },
    error::{ClientExecuteError, ClientInsertWithFormatError, ClientSelectWithFormatError, Error},
};

pub type Settings<'a> = Vec<(&'a str, &'a str)>;

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

#[derive(Debug, Clone)]
pub struct Client {
    http_client: HttpClient,
    client_config: ClientConfig,
}
impl Deref for Client {
    type Target = ClientConfig;

    fn deref(&self) -> &Self::Target {
        &self.client_config
    }
}
impl DerefMut for Client {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.client_config
    }
}
impl Client {
    pub fn new() -> Result<Self, Error> {
        ClientBuilder::default().build()
    }

    pub async fn ping(&self) -> Result<bool, Error> {
        let url = self.get_url();
        let mut req = self.get_request();

        let url = url.join("ping")?;

        *req.method_mut() = Method::GET;
        *req.uri_mut() = url.as_str().parse()?;

        let mut resp = self.http_client.send_async(req).await?;

        if resp.status() != StatusCode::OK {
            return Ok(false);
        }

        let resp_body_text = resp.text().await?;
        Ok(resp_body_text == self.get_http_server_default_response())
    }

    //
    //
    //
    pub async fn execute(
        &self,
        sql: impl AsRef<str>,
        settings: impl Into<Option<Settings<'_>>>,
    ) -> Result<(), Error> {
        let resp = self.respond_execute(sql, settings, |req| req).await?;

        if !resp.status().is_success() {
            return Err(ClientExecuteError::StatusCodeMismatch(resp.status()).into());
        }

        Ok(())
    }

    pub async fn respond_execute<PreRF>(
        &self,
        sql: impl AsRef<str>,
        settings: impl Into<Option<Settings<'_>>>,
        mut pre_respond_fn: PreRF,
    ) -> Result<Response<AsyncBody>, Error>
    where
        PreRF: FnMut(Request<Vec<u8>>) -> Request<Vec<u8>> + Send,
    {
        let mut url = self.get_url().to_owned();
        let mut req = self.get_request();

        if let Some(settings) = settings.into() {
            settings.iter().for_each(|(k, v)| {
                url.query_pairs_mut().append_pair(k, v);
            });
        }

        *req.method_mut() = Method::POST;
        *req.uri_mut() = url.as_str().parse()?;

        let (parts, _) = req.into_parts();
        let req = Request::from_parts(parts, sql.as_ref().as_bytes().to_owned());

        let req = pre_respond_fn(req);

        let resp = self.http_client.send_async(req).await?;

        Ok(resp)
    }

    //
    //
    //
    pub async fn insert_with_format<I: Input>(
        &self,
        sql_prefix: impl AsRef<str>,
        input: I,
        settings: impl Into<Option<Settings<'_>>>,
    ) -> Result<(), Error> {
        let resp = self
            .respond_insert_with_format(sql_prefix, input, settings, |req| req)
            .await?;

        if !resp.status().is_success() {
            return Err(ClientInsertWithFormatError::StatusCodeMismatch(resp.status()).into());
        }

        Ok(())
    }

    pub async fn respond_insert_with_format<I: Input, PreRF>(
        &self,
        sql_prefix: impl AsRef<str>,
        input: I,
        settings: impl Into<Option<Settings<'_>>>,
        pre_respond_fn: PreRF,
    ) -> Result<Response<AsyncBody>, Error>
    where
        PreRF: FnMut(Request<Vec<u8>>) -> Request<Vec<u8>> + Send,
    {
        let format_name = I::format_name();
        let format_bytes = input
            .serialize()
            .map_err(|err| ClientInsertWithFormatError::FormatSerError(err.to_string()))?;

        self.respond_insert_with_format_bytes(
            sql_prefix,
            format_name,
            format_bytes,
            settings,
            pre_respond_fn,
        )
        .await
    }

    pub async fn insert_with_format_bytes(
        &self,
        sql_prefix: impl AsRef<str>,
        format_name: FormatName,
        format_bytes: Vec<u8>,
        settings: impl Into<Option<Settings<'_>>>,
    ) -> Result<(), Error> {
        let resp = self
            .respond_insert_with_format_bytes(
                sql_prefix,
                format_name,
                format_bytes,
                settings,
                |req| req,
            )
            .await?;

        if !resp.status().is_success() {
            return Err(ClientInsertWithFormatError::StatusCodeMismatch(resp.status()).into());
        }

        Ok(())
    }

    pub async fn respond_insert_with_format_bytes<PreRF>(
        &self,
        sql_prefix: impl AsRef<str>,
        format_name: FormatName,
        format_bytes: Vec<u8>,
        settings: impl Into<Option<Settings<'_>>>,
        mut pre_respond_fn: PreRF,
    ) -> Result<Response<AsyncBody>, Error>
    where
        PreRF: FnMut(Request<Vec<u8>>) -> Request<Vec<u8>> + Send,
    {
        let mut url = self.get_url().to_owned();
        let mut req = self.get_request();

        let mut sql = sql_prefix.as_ref().to_owned();
        let sql_suffix = format!(" FORMAT {format_name}");
        if !sql.ends_with(sql_suffix.as_str()) {
            sql.push_str(sql_suffix.as_str());
        }

        url.query_pairs_mut()
            .append_pair(QUERY_KEY_URL_PARAMETER, sql.as_str());

        if let Some(settings) = settings.into() {
            settings.iter().for_each(|(k, v)| {
                url.query_pairs_mut().append_pair(k, v);
            });
        }

        *req.method_mut() = Method::POST;
        *req.uri_mut() = url.as_str().parse()?;

        let (parts, _) = req.into_parts();
        let req = Request::from_parts(parts, format_bytes);

        let req = pre_respond_fn(req);

        let resp = self.http_client.send_async(req).await?;

        Ok(resp)
    }

    //
    //
    //
    pub async fn select_with_format<O: Output>(
        &self,
        sql: impl AsRef<str>,
        output: O,
        settings: impl Into<Option<Settings<'_>>>,
    ) -> Result<(Vec<O::Row>, O::Info), Error> {
        self.internal_select_with_format(sql, output, settings, |req| req)
            .await
            .map(|(_, x)| x)
    }

    pub async fn internal_select_with_format<O: Output, PreRF>(
        &self,
        sql: impl AsRef<str>,
        output: O,
        settings: impl Into<Option<Settings<'_>>>,
        mut pre_respond_fn: PreRF,
    ) -> Result<(ResponseParts, (Vec<O::Row>, O::Info)), Error>
    where
        PreRF: FnMut(Request<Vec<u8>>) -> Request<Vec<u8>> + Send,
    {
        let mut url = self.get_url().to_owned();
        let mut req = self.get_request();

        url.query_pairs_mut().append_pair(
            FORMAT_KEY_URL_PARAMETER,
            O::format_name().to_string().as_str(),
        );

        if let Some(settings) = settings.into() {
            settings.iter().for_each(|(k, v)| {
                url.query_pairs_mut().append_pair(k, v);
            });
        }

        *req.method_mut() = Method::POST;
        *req.uri_mut() = url.as_str().parse()?;

        let (parts, _) = req.into_parts();
        let req = Request::from_parts(parts, sql.as_ref().as_bytes().to_owned());

        let req = pre_respond_fn(req);

        let resp = self.http_client.send_async(req).await?;

        if !resp.status().is_success() {
            return Err(ClientSelectWithFormatError::StatusCodeMismatch(resp.status()).into());
        }

        let resp_format = resp.headers().get(FORMAT_KEY_HEADER);
        if let Some(resp_format) = resp_format
            && resp_format != O::format_name().to_string().as_str()
        {
            return Err(ClientSelectWithFormatError::FormatMismatch(
                resp_format.to_str().unwrap_or("Unknown").to_string(),
            )
            .into());
        }

        let (parts, body) = resp.into_parts();
        let (mut resp_parts, _) = Response::new(()).into_parts();
        resp_parts.status = parts.status;
        resp_parts.version = parts.version;
        resp_parts.headers = parts.headers.to_owned();
        let mut resp = Response::from_parts(parts, body);

        let mut resp_body_buf = Vec::with_capacity(4096);
        resp.copy_to(&mut resp_body_buf).await?;

        let rows_and_info = output
            .deserialize(&resp_body_buf[..])
            .map_err(|err| ClientSelectWithFormatError::FormatDeError(err.to_string()))?;

        Ok((resp_parts, rows_and_info))
    }
}
