use crate::base::BaseShortener;
use crate::error::Error;
use crate::error::Result;
use async_trait::async_trait;
use reqwest::header::AUTHORIZATION;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::collections::HashMap;
use url::Url;
use std::env;


pub struct Shortener {
    pub timeout: u32,
    pub verify: bool,
    pub proxies: BTreeMap<String, String>,
    _api_url: String,
    pub api_key: String,
}

impl Default for Shortener {
    fn default() -> Self {
        Self {
            timeout: 2,
            _api_url: String::from("https://api-ssl.bit.ly/v4"),
            verify: true,
            proxies: BTreeMap::new(),
            api_key: env::var("BITLY_TOKEN").expect("env var BITLY_TOKEN is not set"),
        }
    }
}

#[derive(Serialize, Debug, Deserialize)]
struct Shorten {
    created_at: String,
    id: String,
    link: String,
    custom_bitlinks: Vec<String>,
    long_url: String,
    archived: bool,
    tags: Vec<String>,
    deeplinks: Vec<String>,
    references: HashMap<String, String>,

}

#[async_trait]
impl BaseShortener for Shortener {
    async fn expand(&self, url: &str) -> Result<Url> {
        let url = self.clean_url(url.as_bytes().to_vec()).unwrap_or(vec![]);
        let str_from_url = match std::str::from_utf8(&url) {
            Ok(s) => s,
            Err(e) => return Err(Error::BadUrl(e.to_string())),
        };

        let shorten_url = self._api_url.to_string() + "/shorten";
        let _bitlink_id_header = reqwest::header::HeaderName::from_static("bitlink_id");

        let mut map = HashMap::new();
        map.insert("long_url", str_from_url);   

        let client = reqwest::Client::new();
        let response = client
            .post(shorten_url)
            .header(AUTHORIZATION, "Bearer ".to_string() + &self.api_key)
            .json(&map)
            .send()
            .await;
        match response {
            Ok(r) => {
                match r.json::<Shorten>().await {
                    Ok(shorten) => {
                        match Url::parse(shorten.link.as_str()) {
                            Ok(s) => return Ok(s),
                            Err(e) => return Err(Error::BadUrl(e.to_string()))
                        };
                    }
                    Err(e) => Err(Error::ResponseError(e.to_string()))
                }
            },
            Err(e) => Err(Error::ExpandError(e.to_string())),
        }
    }

    async fn short(&self, url: &str) -> Result<Url> {
        let url = self.clean_url(url.as_bytes().to_vec()).unwrap_or(vec![]);
        let str_from_utf8 = match std::str::from_utf8(&url) {
            Ok(s) => s,
            Err(e) => return Err(Error::BadUrl(e.to_string())),
        };
        let response = self.get(str_from_utf8).await;
        match response {
            Ok(r) => Ok(r.url().to_owned()),
            Err(e) => Err(Error::ExpandError(e.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::base::BaseShortener;

    use super::Shortener;

    const SHORTEN: &str = "https://bit.ly/3e43fWI";
    const HTTPS_EXPANDED: &str = "https://www.google.com/";
    const EXPANDED: &str = "www.google.com/";

    #[tokio::test(flavor = "multi_thread")]
    async fn test_bitly_expand_without_scheme() {
        let bitly: Shortener = Default::default();
        let bitly = bitly.expand(EXPANDED).await.unwrap();
        assert_eq!(bitly.as_str(), SHORTEN);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_bitly_expand() {
        let bitly: Shortener = Default::default();
        let bitly = bitly.expand(HTTPS_EXPANDED).await.unwrap();
        assert_eq!(bitly.as_str(), SHORTEN);
    }

}
