use isahc::http::Request;
use once_cell::sync::Lazy;
use url::Url;

use crate::error::Error;

static URL_DEFAULT: Lazy<Url> = Lazy::new(|| Url::parse("http://localhost:8123/").unwrap());
const DATABASE_KEY_URL_PARAMETER: &str = "database";
const DATABASE_KEY_HEADER: &str = "X-ClickHouse-Database";
const USERNAME_KEY_URL_PARAMETER: &str = "user";
const USERNAME_KEY_HEADER: &str = "X-ClickHouse-User";
const PASSWORD_KEY_URL_PARAMETER: &str = "password";
const PASSWORD_KEY_HEADER: &str = "X-ClickHouse-Key";

#[derive(Debug, Clone)]
pub enum ClientConfigLocation {
    UrlParameter,
    Header,
}
impl Default for ClientConfigLocation {
    fn default() -> Self {
        Self::UrlParameter
    }
}

#[derive(Default, Debug, Clone)]
pub struct ClientConfig {
    url: Option<Url>,
    database: Option<(String, ClientConfigLocation)>,
    credentials: Option<(String, Option<String>, ClientConfigLocation)>,
}
impl ClientConfig {
    pub fn set_url(&mut self, url: impl Into<String>) -> Result<&mut Self, Error> {
        self.url = Some(url.into().parse()?);
        Ok(self)
    }
    pub fn set_database(
        &mut self,
        db_name: impl Into<String>,
        to: impl Into<Option<ClientConfigLocation>>,
    ) -> &mut Self {
        self.database = Some((db_name.into(), to.into().unwrap_or_default()));
        self
    }
    pub fn set_credentials(
        &mut self,
        username: impl Into<String>,
        password: impl Into<Option<String>>,
        to: impl Into<Option<ClientConfigLocation>>,
    ) -> &mut Self {
        self.credentials = Some((
            username.into(),
            password.into(),
            to.into().unwrap_or_default(),
        ));
        self
    }
}
impl ClientConfig {
    pub(crate) fn get_url_and_request(&self) -> Result<(Url, Request<()>), Error> {
        let mut url = self
            .url
            .to_owned()
            .unwrap_or_else(|| URL_DEFAULT.to_owned());
        let mut req = Request::default();

        if let Some((db_name, to)) = &self.database {
            match to {
                ClientConfigLocation::UrlParameter => {
                    url.query_pairs_mut()
                        .append_pair(DATABASE_KEY_URL_PARAMETER, db_name);
                }
                ClientConfigLocation::Header => {
                    req.headers_mut()
                        .append(DATABASE_KEY_HEADER, db_name.parse()?);
                }
            }
        }
        if let Some((username, password, to)) = &self.credentials {
            match to {
                ClientConfigLocation::UrlParameter => {
                    url.query_pairs_mut()
                        .append_pair(USERNAME_KEY_URL_PARAMETER, username);
                }
                ClientConfigLocation::Header => {
                    req.headers_mut()
                        .append(USERNAME_KEY_HEADER, username.parse()?);
                }
            }
            if let Some(password) = password {
                match to {
                    ClientConfigLocation::UrlParameter => {
                        url.query_pairs_mut()
                            .append_pair(PASSWORD_KEY_URL_PARAMETER, password);
                    }
                    ClientConfigLocation::Header => {
                        req.headers_mut()
                            .append(PASSWORD_KEY_HEADER, password.parse()?);
                    }
                }
            }
        }

        Ok((url, req))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::error;

    #[test]
    fn with_default() -> Result<(), Box<dyn error::Error>> {
        let (url, req) = ClientConfig::default().get_url_and_request()?;
        assert_eq!(url.as_str(), "http://localhost:8123/");
        assert_eq!(req.headers().len(), 0);

        Ok(())
    }

    #[test]
    fn with_set_url() -> Result<(), Box<dyn error::Error>> {
        let (url, req) = ClientConfig::default()
            .set_url("http://127.0.0.1:8123/foo/?bar=1")?
            .get_url_and_request()?;
        assert_eq!(url.as_str(), "http://127.0.0.1:8123/foo/?bar=1");
        assert_eq!(req.headers().len(), 0);

        Ok(())
    }

    #[test]
    fn with_set_database() -> Result<(), Box<dyn error::Error>> {
        let (url, req) = ClientConfig::default()
            .set_url("http://127.0.0.1:8123/foo/?bar=1")?
            .set_database("db", ClientConfigLocation::UrlParameter)
            .get_url_and_request()?;
        assert_eq!(url.as_str(), "http://127.0.0.1:8123/foo/?bar=1&database=db");
        assert_eq!(req.headers().len(), 0);

        let (url, req) = ClientConfig::default()
            .set_database("db", ClientConfigLocation::Header)
            .get_url_and_request()?;
        assert_eq!(url.as_str(), "http://localhost:8123/");
        assert_eq!(req.headers().len(), 1);
        assert_eq!(req.headers().get("X-ClickHouse-Database").unwrap(), "db");

        Ok(())
    }

    #[test]
    fn with_set_credentials() -> Result<(), Box<dyn error::Error>> {
        let (url, req) = ClientConfig::default()
            .set_url("http://127.0.0.1:8123/foo/?bar=1")?
            .set_credentials("user", None, ClientConfigLocation::UrlParameter)
            .get_url_and_request()?;
        assert_eq!(url.as_str(), "http://127.0.0.1:8123/foo/?bar=1&user=user");
        assert_eq!(req.headers().len(), 0);

        let (url, req) = ClientConfig::default()
            .set_url("http://127.0.0.1:8123/foo/?bar=1")?
            .set_credentials(
                "user",
                "password".to_owned(),
                ClientConfigLocation::UrlParameter,
            )
            .get_url_and_request()?;
        assert_eq!(
            url.as_str(),
            "http://127.0.0.1:8123/foo/?bar=1&user=user&password=password"
        );
        assert_eq!(req.headers().len(), 0);

        let (url, req) = ClientConfig::default()
            .set_credentials("user", None, ClientConfigLocation::Header)
            .get_url_and_request()?;
        assert_eq!(url.as_str(), "http://localhost:8123/");
        assert_eq!(req.headers().len(), 1);
        assert_eq!(req.headers().get("X-ClickHouse-User").unwrap(), "user");

        let (url, req) = ClientConfig::default()
            .set_credentials("user", "password".to_owned(), ClientConfigLocation::Header)
            .get_url_and_request()?;
        assert_eq!(url.as_str(), "http://localhost:8123/");
        assert_eq!(req.headers().len(), 2);
        assert_eq!(req.headers().get("X-ClickHouse-User").unwrap(), "user");
        assert_eq!(req.headers().get("X-ClickHouse-Key").unwrap(), "password");

        Ok(())
    }
}
