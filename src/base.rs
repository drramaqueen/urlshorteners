use async_trait::async_trait;
use url::Url;

use crate::error::Error;
use crate::error::Result;

#[async_trait]
pub trait BaseShortener {
    async fn get(&self, url: &str) -> Result<reqwest::Response> {
        match reqwest::get(url).await {
            Ok(r) => Ok(r),
            Err(e) => Err(Error::ResponseError(e.to_string())),
        }
    }

    async fn post(&self, url: &str, data: Vec<u8>) -> Result<()> {
        let client = reqwest::Client::new();
        match client.post(url).body(data).send().await {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::ResponseError(e.to_string())),
        }
    }

    async fn expand(&self, url: &str) -> Result<Url> {
        let response = self.get(url).await;
        match response {
            Ok(r) => Ok(r.url().to_owned()),
            Err(e) => Err(Error::ExpandError(e.to_string())),
        }
    }

    async fn short(&self, url: &str) -> Result<Url>;

    fn clean_url(&self, url: Vec<u8>) -> Result<Vec<u8>> {
        let str_from_url = match std::str::from_utf8(&url) {
            Ok(s) => s,
            Err(e) => return Err(Error::BadUrl(e.to_string())),
        };
        let new_url: Vec<u8> = if !url.starts_with(b"http://") && !url.starts_with(b"https://") {
            ("https://".to_string() + str_from_url).as_bytes().to_vec()
        } else {
            str_from_url.as_bytes().to_vec()
        };
        let new_url_from_utf8 = match std::str::from_utf8(&new_url) {
            Ok(s) => s,
            Err(e) => return Err(Error::BadUrl(e.to_string())),
        };
        let new_url = match Url::parse(new_url_from_utf8) {
            Ok(_) => new_url,
            Err(e) => return Err(Error::BadUrl(e.to_string())),
        };
        Ok(new_url)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        base::BaseShortener,
        bitly::{self},
    };
    #[tokio::test(flavor = "multi_thread")]
    async fn test_clean_url() {
        let b = bitly::Shortener::default();

        let clean_url = b.clean_url("google.com".as_bytes().to_vec()).unwrap();
        assert_eq!(
            std::str::from_utf8(&clean_url).unwrap(),
            "https://google.com"
        );
    }
}
