use isahc::http::{
    header::{HeaderMap, InvalidHeaderValue},
    Request,
};
use url::{ParseError, Url};

pub const DATABASE_KEY_URL_PARAMETER: &str = "database";
pub const DATABASE_KEY_HEADER: &str = "X-ClickHouse-Database";

pub const USERNAME_KEY_URL_PARAMETER: &str = "user";
pub const USERNAME_KEY_HEADER: &str = "X-ClickHouse-User";

pub const PASSWORD_KEY_URL_PARAMETER: &str = "password";
pub const PASSWORD_KEY_HEADER: &str = "X-ClickHouse-Key";

pub const FORMAT_KEY_URL_PARAMETER: &str = "default_format";
pub const FORMAT_KEY_HEADER: &str = "X-ClickHouse-Format";

pub const QUERY_KEY_URL_PARAMETER: &str = "query";

pub const SUMMARY_KEY_HEADER: &str = "X-ClickHouse-Summary";
pub const QUERY_ID_KEY_HEADER: &str = "X-ClickHouse-Query-Id";

const HTTP_SERVER_DEFAULT_RESPONSE_DEFAULT: &str = "Ok.\n";

#[derive(Debug, Clone)]
pub struct ClientConfig {
    url: Url,
    header_map: HeaderMap,
    http_server_default_response: Option<String>,
}
impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            url: "http://localhost:8123/".parse().unwrap(),
            header_map: HeaderMap::new(),
            http_server_default_response: None,
        }
    }
}
impl ClientConfig {
    pub fn set_url(&mut self, val: impl AsRef<str>) -> Result<&mut Self, ParseError> {
        let mut query_pairs = self.url.query_pairs();
        let database = query_pairs
            .find(|(k, _)| k == DATABASE_KEY_URL_PARAMETER)
            .map(|(_, v)| v.to_string());
        let username = query_pairs
            .find(|(k, _)| k == USERNAME_KEY_URL_PARAMETER)
            .map(|(_, v)| v.to_string());
        let password = query_pairs
            .find(|(k, _)| k == PASSWORD_KEY_URL_PARAMETER)
            .map(|(_, v)| v.to_string());

        let mut url: Url = val.as_ref().parse()?;
        if let Some(ref database) = database {
            url.query_pairs_mut()
                .append_pair(DATABASE_KEY_URL_PARAMETER, database);
        }
        if let Some(ref username) = username {
            url.query_pairs_mut()
                .append_pair(USERNAME_KEY_URL_PARAMETER, username);
        }
        if let Some(ref password) = password {
            url.query_pairs_mut()
                .append_pair(PASSWORD_KEY_URL_PARAMETER, password);
        }

        self.url = url;
        Ok(self)
    }
    pub fn set_database_to_url_parameter(&mut self, val: impl AsRef<str>) -> &mut Self {
        self.url
            .query_pairs_mut()
            .append_pair(DATABASE_KEY_URL_PARAMETER, val.as_ref());
        self
    }
    pub fn set_database_to_header(
        &mut self,
        val: impl AsRef<str>,
    ) -> Result<&mut Self, InvalidHeaderValue> {
        self.header_map
            .insert(DATABASE_KEY_HEADER, val.as_ref().parse()?);
        Ok(self)
    }

    pub fn set_username_to_url_parameter(&mut self, val: impl AsRef<str>) -> &mut Self {
        self.url
            .query_pairs_mut()
            .append_pair(USERNAME_KEY_URL_PARAMETER, val.as_ref());
        self
    }
    pub fn set_username_to_header(
        &mut self,
        val: impl AsRef<str>,
    ) -> Result<&mut Self, InvalidHeaderValue> {
        self.header_map
            .insert(USERNAME_KEY_HEADER, val.as_ref().parse()?);
        Ok(self)
    }

    pub fn set_password_to_url_parameter(&mut self, val: impl AsRef<str>) -> &mut Self {
        self.url
            .query_pairs_mut()
            .append_pair(PASSWORD_KEY_URL_PARAMETER, val.as_ref());
        self
    }
    pub fn set_password_to_header(
        &mut self,
        val: impl AsRef<str>,
    ) -> Result<&mut Self, InvalidHeaderValue> {
        self.header_map
            .insert(PASSWORD_KEY_HEADER, val.as_ref().parse()?);
        Ok(self)
    }

    pub fn set_http_server_default_response(&mut self, val: impl Into<String>) -> &mut Self {
        self.http_server_default_response = Some(val.into());
        self
    }
}
impl ClientConfig {
    pub(crate) fn get_url(&self) -> &Url {
        &self.url
    }
    pub(crate) fn get_request(&self) -> Request<()> {
        let mut req = Request::new(());

        *req.headers_mut() = self.header_map.to_owned();

        req
    }
    pub(crate) fn get_http_server_default_response(&self) -> &str {
        self.http_server_default_response
            .as_deref()
            .unwrap_or(HTTP_SERVER_DEFAULT_RESPONSE_DEFAULT)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_default() -> Result<(), Box<dyn std::error::Error>> {
        let config = ClientConfig::default();
        assert_eq!(config.url.as_str(), "http://localhost:8123/");
        assert_eq!(config.header_map.len(), 0);

        Ok(())
    }

    #[test]
    fn with_set_url() -> Result<(), Box<dyn std::error::Error>> {
        let mut config = ClientConfig::default();
        config.set_url("http://127.0.0.1:8123/foo/?bar=1")?;
        assert_eq!(config.url.as_str(), "http://127.0.0.1:8123/foo/?bar=1");
        assert_eq!(config.header_map.len(), 0);

        Ok(())
    }

    #[test]
    fn with_set_database() -> Result<(), Box<dyn std::error::Error>> {
        let mut config = ClientConfig::default();
        config
            .set_url("http://127.0.0.1:8123/foo/?bar=1")?
            .set_database_to_url_parameter("db");
        assert_eq!(
            config.url.as_str(),
            "http://127.0.0.1:8123/foo/?bar=1&database=db"
        );
        assert_eq!(config.header_map.len(), 0);

        let mut config = ClientConfig::default();
        config.set_database_to_header("db")?;
        assert_eq!(config.url.as_str(), "http://localhost:8123/");
        assert_eq!(config.header_map.len(), 1);
        assert_eq!(
            config.header_map.get("X-ClickHouse-Database").unwrap(),
            "db"
        );

        Ok(())
    }

    #[test]
    fn with_set_username() -> Result<(), Box<dyn std::error::Error>> {
        let mut config = ClientConfig::default();
        config
            .set_url("http://127.0.0.1:8123/foo/?bar=1")?
            .set_username_to_url_parameter("user");
        assert_eq!(
            config.url.as_str(),
            "http://127.0.0.1:8123/foo/?bar=1&user=user"
        );
        assert_eq!(config.header_map.len(), 0);

        let mut config = ClientConfig::default();
        config.set_username_to_header("user")?;
        assert_eq!(config.url.as_str(), "http://localhost:8123/");
        assert_eq!(config.header_map.len(), 1);
        assert_eq!(config.header_map.get("X-ClickHouse-User").unwrap(), "user");

        Ok(())
    }

    #[test]
    fn with_set_password() -> Result<(), Box<dyn std::error::Error>> {
        let mut config = ClientConfig::default();
        config
            .set_url("http://127.0.0.1:8123/foo/?bar=1")?
            .set_password_to_url_parameter("password");
        assert_eq!(
            config.url.as_str(),
            "http://127.0.0.1:8123/foo/?bar=1&password=password"
        );
        assert_eq!(config.header_map.len(), 0);

        let mut config = ClientConfig::default();
        config.set_password_to_header("password")?;
        assert_eq!(config.url.as_str(), "http://localhost:8123/");
        assert_eq!(config.header_map.len(), 1);
        assert_eq!(
            config.header_map.get("X-ClickHouse-Key").unwrap(),
            "password"
        );

        Ok(())
    }
}
