use crate::base::BaseShortener;
use crate::error::Error;
use crate::error::Result;
use async_trait::async_trait;
use reqwest::header::AUTHORIZATION;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use url::Url;

pub struct Shortener {
    pub timeout: u32,
    pub verify: bool,
    _api_url: String,
    pub api_key: String,
}

impl Default for Shortener {
    fn default() -> Self {
        Self {
            timeout: 2,
            _api_url: String::from("https://api-ssl.bit.ly/v4"),
            verify: true,
            api_key: env::var("BITLY_TOKEN").expect("env var BITLY_TOKEN is not set"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Short {
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

#[derive(Debug, Deserialize, Serialize)]
struct Expand {
    link: String,
    id: String,
    long_url: String,
    created_at: String,
}

#[async_trait]
impl BaseShortener for Shortener {
    async fn shorten(&self, url: &str) -> Result<Url> {
        let url = self.clean_url(url.as_bytes().to_vec()).unwrap_or_default();
        let str_from_url = match std::str::from_utf8(&url) {
            Ok(s) => s,
            Err(e) => return Err(Error::BadUrl(e.to_string())),
        };

        let shorten_url = self._api_url.to_string() + "/shorten";
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
            Ok(r) => match r.json::<Short>().await {
                Ok(shorten) => {
                    match Url::parse(shorten.link.as_str()) {
                        Ok(s) => return Ok(s),
                        Err(e) => return Err(Error::BadUrl(e.to_string())),
                    };
                }
                Err(e) => Err(Error::ResponseError(e.to_string())),
            },
            Err(e) => Err(Error::ShortenError(e.to_string())),
        }
    }

    async fn expand(&self, url: &str) -> Result<Url> {
        let str_from_url = match std::str::from_utf8(url.as_bytes()) {
            Ok(s) => s,
            Err(e) => return Err(Error::BadUrl(e.to_string())),
        };

        let expand_url = self._api_url.to_string() + "/expand";

        let mut map = HashMap::new();
        map.insert("bitlink_id", str_from_url);

        let client = reqwest::Client::new();
        let response = client
            .post(expand_url)
            .header(AUTHORIZATION, "Bearer ".to_string() + &self.api_key)
            .json(&map)
            .send()
            .await;
        match response {
            Ok(r) => match r.json::<Expand>().await {
                Ok(expanded) => {
                    match Url::parse(expanded.long_url.as_str()) {
                        Ok(s) => return Ok(s),
                        Err(e) => return Err(Error::BadUrl(e.to_string())),
                    };
                }
                Err(e) => Err(Error::ResponseError(e.to_string())),
            },
            Err(e) => Err(Error::ShortenError(e.to_string())),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Clicks {
    link_clicks: Vec<LinkClicks>,
    units: i32,
    unit: String,
    unit_reference: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LinkClicks {
    clicks: u32,
    date: String,
}

impl Shortener {
    #[allow(dead_code)]
    async fn link_clicks(&self, url: &str) -> Result<Vec<LinkClicks>> {
        let clicks_url = self._api_url.to_string() + "/bitlinks/" + url + "/clicks";
        let client = reqwest::Client::new();
        let response = client
            .get(clicks_url)
            .header(AUTHORIZATION, "Bearer ".to_string() + &self.api_key)
            .send()
            .await;

        match response {
            Ok(r) => match r.json::<Clicks>().await {
                Ok(clicks) => {
                    println!("OK {:?}", clicks);
                    Ok(clicks.link_clicks)
                }
                Err(e) => Err(Error::ResponseError(e.to_string())),
            },
            Err(e) => Err(Error::ResponseError(e.to_string())),
        }
    }

    #[allow(dead_code)]
    async fn link_clicks_total_count(&self, url: &str) -> Result<u32> {
        let mut res = 0;
        let clicks = self.link_clicks(url).await;
        match clicks {
            Ok(info) => {
                for i in info {
                    res += i.clicks;
                }
                Ok(res)
            }
            Err(e) => Err(Error::ResponseError(e.to_string())),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::Shortener;
    use crate::base::BaseShortener;

    const HTTPS_SHORTEN: &str = "https://bit.ly/3e43fWI";
    const SHORTEN: &str = "bit.ly/3e43fWI";
    const HTTPS_EXPANDED: &str = "https://www.google.com/";
    const EXPANDED: &str = "www.google.com/";

    #[tokio::test(flavor = "multi_thread")]
    async fn test_bitly_shorten_without_scheme() {
        let bitly: Shortener = Default::default();
        let bitly = bitly.shorten(EXPANDED).await.unwrap();
        assert_eq!(bitly.as_str(), HTTPS_SHORTEN);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_bitly_shorten() {
        let bitly: Shortener = Default::default();
        let bitly = bitly.shorten(HTTPS_EXPANDED).await.unwrap();
        assert_eq!(bitly.as_str(), HTTPS_SHORTEN);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_bitly_expand() {
        let bitly: Shortener = Default::default();
        let expanded = bitly.expand(SHORTEN).await.unwrap();
        assert_eq!(expanded.as_str(), HTTPS_EXPANDED);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_bitly_clicks() {
        let bitly: Shortener = Default::default();
        let clicks = bitly.link_clicks(SHORTEN).await;
        assert!(clicks.unwrap().len() > 0);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_bitly_clicks_total_count() {
        let bitly: Shortener = Default::default();
        let clicks = bitly.link_clicks_total_count(SHORTEN).await;
        assert!(clicks.unwrap() > 0);
    }
}
